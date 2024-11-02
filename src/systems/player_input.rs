use crate::prelude::*;

#[system]
#[read_component(Carried)]
#[read_component(Enemy)]
#[read_component(Item)]
#[read_component(Player)]
#[read_component(Point)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let (player, pos) = get_player_info(ecs);
    let destination = key.and_then(delta).map(|delta| delta + pos);
    let attacks = gather_attacks(ecs, player, destination);
    let action = determine_action(player, attacks, destination, key, ecs);
    let take_turn = action != Action::None;

    match action {
        Action::ActivateItem(a) => {
            commands.extend(a);
        }
        Action::Attack(a) => {
            commands.extend(a);
        }
        Action::GetMagicItem => pick_up(ecs, player, pos, commands),
        // note: heal should also be a command?
        Action::Heal => heal(ecs, player),
        Action::Move(m) => {
            commands.extend(m);
        }
        Action::None => (),
        Action::ShowPlayerPosition => println!(">>>Player at {:?}", pos),
    };

    if take_turn {
        *turn_state = TurnState::PlayerTurn;
    }
}

fn delta(key: VirtualKeyCode) -> Option<Point> {
    match key {
        VirtualKeyCode::Left => Some(Point::new(-1, 0)),
        VirtualKeyCode::Right => Some(Point::new(1, 0)),
        VirtualKeyCode::Up => Some(Point::new(0, -1)),
        VirtualKeyCode::Down => Some(Point::new(0, 1)),
        _ => None,
    }
}

fn determine_action(
    player: Entity,
    attacks: AttackCommandVec,
    destination: Option<Point>,
    key: &Option<VirtualKeyCode>,
    ecs: &SubWorld,
) -> Action {
    if key.is_none() {
        return Action::None;
    }

    if !attacks.is_empty() {
        return Action::Attack(attacks);
    }

    if let Some(destination) = destination {
        return Action::Move(vec![((), WantsToMove::new(player, destination))]);
    }

    let key = key.unwrap();
    match key {
        VirtualKeyCode::G => Action::GetMagicItem,
        VirtualKeyCode::P => Action::ShowPlayerPosition,
        VirtualKeyCode::Key1 => Action::ActivateItem(select_item(0, player, ecs)),
        VirtualKeyCode::Key2 => Action::ActivateItem(select_item(1, player, ecs)),
        VirtualKeyCode::Key3 => Action::ActivateItem(select_item(2, player, ecs)),
        VirtualKeyCode::Key4 => Action::ActivateItem(select_item(3, player, ecs)),
        VirtualKeyCode::Key5 => Action::ActivateItem(select_item(4, player, ecs)),
        VirtualKeyCode::Key6 => Action::ActivateItem(select_item(5, player, ecs)),
        VirtualKeyCode::Key7 => Action::ActivateItem(select_item(6, player, ecs)),
        VirtualKeyCode::Key8 => Action::ActivateItem(select_item(7, player, ecs)),
        VirtualKeyCode::Key9 => Action::ActivateItem(select_item(8, player, ecs)),
        _ => Action::Heal,
    }
}

fn gather_attacks(ecs: &SubWorld, player: Entity, destination: Option<Point>) -> AttackCommandVec {
    destination
        .map(|destination| {
            <(Entity, &Point)>::query()
                .filter(component::<Enemy>())
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .map(|(entity, _)| ((), WantsToAttack::new(player, *entity)))
                .collect()
        })
        .unwrap_or_default()
}

fn get_player_info(ecs: &SubWorld) -> (Entity, Point) {
    <(Entity, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .map(|(player, pos)| (*player, *pos))
        .unwrap()
}

fn heal(ecs: &mut SubWorld, player: Entity) {
    if let Ok(health) = ecs.entry_mut(player).unwrap().get_component_mut::<Health>() {
        if health.current < health.max {
            health.current += 1;
            println!("Healed to {}", health.current);
        }
    }
}

fn pick_up(ecs: &mut SubWorld, player: Entity, pos: Point, commands: &mut CommandBuffer) {
    <(Entity, &Item, &Point)>::query()
        .iter(ecs)
        .filter(|(.., &item_pos)| item_pos == pos)
        .for_each(|(entity, ..)| {
            commands.remove_component::<Point>(*entity);
            commands.add_component(*entity, Carried(player));
        });
}

fn select_item(n: usize, player_entity: Entity, ecs: &SubWorld) -> ActivateItemCommandVec {
    let item_entity = <(Entity, &Item, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player_entity)
        .enumerate()
        .filter(|(item_count, (_, _, _))| *item_count == n)
        .find_map(|(_, (&item_entity, _, _))| Some(item_entity));

    if let Some(item_entity) = item_entity {
        return vec![((), ActivateItem::new(item_entity, player_entity))];
    } else {
        Vec::new()
    }
}

#[derive(PartialEq)]
enum Action {
    ActivateItem(ActivateItemCommandVec),
    Attack(AttackCommandVec),
    GetMagicItem,
    Heal,
    Move(MoveCommandVec),
    None,
    ShowPlayerPosition,
}

type ActivateItemCommandVec = Vec<((), ActivateItem)>;
type AttackCommandVec = Vec<((), WantsToAttack)>;
type MoveCommandVec = Vec<((), WantsToMove)>;
