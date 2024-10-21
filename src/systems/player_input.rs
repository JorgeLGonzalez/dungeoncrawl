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
    let (player, pos) = <(Entity, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .map(|(player, pos)| (*player, *pos))
        .unwrap();
    let destination = key.and_then(delta).map(|delta| delta + pos);
    let attacks = gather_attacks(ecs, player, destination);

    if attacks.is_empty() {
        destination.map(|d| {
            commands.push(((), WantsToMove::new(d, player)));
        });
    } else {
        commands.extend(attacks);
    }

    if key.is_some() {
        if destination.is_none() {
            heal(ecs, player);
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

fn gather_attacks(
    ecs: &SubWorld,
    player: Entity,
    destination: Option<Point>,
) -> Vec<((), WantsToAttack)> {
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

fn heal(ecs: &mut SubWorld, player: Entity) {
    if let Ok(health) = ecs.entry_mut(player).unwrap().get_component_mut::<Health>() {
        if health.current < health.max {
            health.current += 1;
            println!("Healed to {}", health.current);
        }
    }
}
