use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(MovingRandomly)]
#[read_component(Player)]
#[read_component(Point)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut rng = RandomNumberGenerator::new();
    let mut positions = <(Entity, &Point, &Health)>::query();

    <(Entity, &Point, &MovingRandomly)>::query()
        .iter(ecs)
        .for_each(|(entity, pos, _)| {
            let destination = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            } + *pos;

            let mut attacked = false;
            positions
                .iter(ecs)
                .filter(|(_, target_pos, _)| **target_pos == destination)
                .for_each(|(victim, _, _)| {
                    if ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        commands.push(((), WantsToAttack::new(*entity, *victim)));
                    }

                    attacked = true;
                });

            if !attacked {
                commands.push(((), WantsToMove::new(destination, *entity)));
            }
        });
}
