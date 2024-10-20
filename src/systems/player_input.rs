use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Point)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        let (player, destination) = <(Entity, &Point)>::query()
            .filter(component::<Player>())
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        let will_move = delta.x != 0 || delta.y != 0;
        if will_move {
            let attacks: Vec<((), WantsToAttack)> = <(Entity, &Point)>::query()
                .filter(component::<Enemy>())
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .map(|(entity, _)| ((), WantsToAttack::new(player, *entity)))
                .collect();

            if attacks.is_empty() {
                commands.push(((), WantsToMove::new(destination, player)));
            } else {
                commands.extend(attacks);
            }
        }
        *turn_state = TurnState::PlayerTurn;
    }
}
