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
    let action = determine_action(player, attacks, destination, key);
    let take_turn = action != Action::None;

    match action {
        Action::Attack(a) => {
            commands.extend(a);
        }
        Action::GetMagicItem => {
            let mut items = <(Entity, &Item, &Point)>::query();
            items
                .iter(ecs)
                .filter(|(_entity, _item, &item_pos)| item_pos == pos)
                .for_each(|(entity, _item, _item_pos)| {
                    commands.remove_component::<Point>(*entity);
                    commands.add_component(*entity, Carried(player));
                });
        }
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
) -> Action {
    if key.is_none() {
        return Action::None;
    }

    if matches!(key, Some(VirtualKeyCode::G)) {
        return Action::GetMagicItem;
    }

    if matches!(key, Some(VirtualKeyCode::P)) {
        return Action::ShowPlayerPosition;
    }

    if !attacks.is_empty() {
        return Action::Attack(attacks);
    }

    if let Some(destination) = destination {
        return Action::Move(vec![((), WantsToMove::new(player, destination))]);
    }

    return Action::Heal;
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

#[derive(PartialEq)]
enum Action {
    Attack(AttackCommandVec),
    GetMagicItem,
    Heal,
    Move(MoveCommandVec),
    None,
    ShowPlayerPosition,
}

type AttackCommandVec = Vec<((), WantsToAttack)>;
type MoveCommandVec = Vec<((), WantsToMove)>;
