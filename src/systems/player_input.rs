use crate::prelude::*;

#[system]
#[read_component(Enemy)]
#[read_component(Player)]
#[read_component(Point)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut did_something = false;
    key.and_then(delta).map(|delta| {
        let player = get_player_info(ecs, delta);
        let attacks = gather_attacks(ecs, &player);

        if attacks.is_empty() {
            commands.push(((), WantsToMove::new(player.destination, player.player)));
        } else {
            commands.extend(attacks);
        }

        did_something = true;
    });

    if key.is_some() {
        let player = <(Entity, &Point)>::query()
            .filter(component::<Player>())
            .iter(ecs)
            .nth(0)
            .map(|(p, _)| *p)
            .unwrap();

        if !did_something {
            if let Ok(health) = ecs.entry_mut(player).unwrap().get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + 1);
            }
        }

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

fn gather_attacks(ecs: &SubWorld, player_info: &PlayerInfo) -> Vec<((), WantsToAttack)> {
    let PlayerInfo {
        destination,
        player,
    } = *player_info;

    <(Entity, &Point)>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        .filter(|(_, pos)| **pos == destination)
        .map(|(entity, _)| ((), WantsToAttack::new(player, *entity)))
        .collect()
}

fn get_player_info(ecs: &SubWorld, delta: Point) -> PlayerInfo {
    <(Entity, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .find_map(|(entity, pos)| Some(PlayerInfo::new(*entity, *pos + delta)))
        .unwrap()
}

struct PlayerInfo {
    destination: Point,
    player: Entity,
}

impl PlayerInfo {
    fn new(player: Entity, destination: Point) -> Self {
        Self {
            destination,
            player,
        }
    }
}
