use super::helpers::{ChaseAction, ChaseActionDeterminer};
use crate::prelude::*;

pub fn chasing(
    mut move_events: EventWriter<WantsToMove>,
    mut attack_events: EventWriter<WantsToAttack>,
    movers: Query<(Entity, &PointC, &FieldOfView), With<ChasingPlayer>>,
    positions: Query<(Entity, &PointC), With<Health>>,
    player: Query<(Entity, &PointC), With<Player>>,
    map: Res<Map>,
) {
    let determiner = ChaseActionDeterminer::new(player, positions, map.as_ref());

    movers
        .iter()
        .filter_map(|(entity, pos, fov)| determiner.determine(entity, pos, fov))
        .for_each(|action| match action {
            ChaseAction::Attack(a) => attack_events.send(a),
            ChaseAction::Move(m) => move_events.send(m),
        })
}
