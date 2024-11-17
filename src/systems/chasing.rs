// use super::helpers::{ChaseAction, ChaseActionDeterminer};
// use crate::prelude::*;

// #[system]
// #[read_component(ChasingPlayer)]
// #[read_component(FieldOfView)]
// #[read_component(Health)]
// #[read_component(Player)]
// #[read_component(Point)]
// pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
//     let mut determiner = ChaseActionDeterminer::new(ecs, map);

//     <(Entity, &Point, &ChasingPlayer, &FieldOfView)>::query()
//         .iter(ecs)
//         .filter_map(|(entity, pos, _, fov)| determiner.determine(*entity, *pos, fov))
//         .for_each(|a| match a {
//             ChaseAction::Attack(a) => {
//                 commands.push(((), a));
//             }
//             ChaseAction::Move(m) => {
//                 commands.push(((), m));
//             }
//         });
// }
