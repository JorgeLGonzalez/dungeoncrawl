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
        <(Entity, &Point)>::query()
            .filter(component::<Player>())
            .iter(ecs)
            .for_each(|(entity, pos)| {
                let destination = *pos + delta;
                commands.push(((), WantsToMove::new(destination, *entity)));
            });

        *turn_state = TurnState::PlayerTurn;
    }
}
