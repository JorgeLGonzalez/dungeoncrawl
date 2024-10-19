use crate::prelude::*;

#[system]
#[read_component(Enemy)]
#[read_component(Player)]
#[read_component(Point)]
pub fn collisions(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let player_pos = *<&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .expect("No player!");

    <(Entity, &Point)>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_pos)
        .for_each(|(entity, _)| {
            commands.remove(*entity);
        })
}
