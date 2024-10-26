use super::helpers::{ChaseAction, ChaseActionDeterminer};
use crate::prelude::*;

#[system]
#[read_component(ChasingPlayer)]
#[read_component(FieldOfView)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let determiner = ChaseActionDeterminer::new(ecs, map);

    let mut planned_moves: Vec<WantsToMove> = Vec::new();

    <(Entity, &Point, &ChasingPlayer, &FieldOfView)>::query()
        .iter(ecs)
        .filter_map(|(entity, pos, _, fov)| determiner.determine(*entity, *pos, fov))
        .for_each(|a| match a {
            ChaseAction::Attack(a) => {
                commands.push(((), a));
            }
            ChaseAction::Move(m) => {
                if !will_be_occupied(&planned_moves, m) {
                    commands.push(((), m));
                    planned_moves.push(m);
                }
            }
        });
}

// see README.md#issue-monsters-able-to-move-on-top-of-each-other)
fn will_be_occupied(planned_moves: &[WantsToMove], this_move: WantsToMove) -> bool {
    let will_be_occupied = planned_moves
        .iter()
        .find(|pm| pm.destination == this_move.destination)
        .is_some();

    if will_be_occupied {
        println!(
            ">>>> Ignoring move to {:?} for {:?} already planned by another monster.",
            this_move.destination, this_move.entity
        )
    }

    will_be_occupied
}
