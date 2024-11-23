// use super::helpers::{ChaseAction, ChaseActionDeterminer};
use crate::prelude::*;

pub fn chasing(
    mut move_events: EventWriter<WantsToMove>,
    mut attack_events: EventWriter<WantsToAttack>,
    movers: Query<(Entity, &PointC, &FieldOfView), With<ChasingPlayer>>,
    positions: Query<(Entity, &PointC), With<Health>>,
    player: Query<&PointC, With<Player>>,
    map: Res<Map>,
) {
    let player_pos = player.single().0;
    let player_idx = map_idx(player_pos.x, player_pos.y);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &search_targets,
        map.as_ref(),
        1024.0,
    );

    for (entity, pos, fov) in movers.iter() {
        if !fov.visible_tiles.contains(&player_pos) {
            continue;
        }

        let idx = map_idx(pos.0.x, pos.0.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map.as_ref()) {
            let distance = DistanceAlg::Pythagoras.distance2d(pos.0, player_pos);
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                player_pos
            };

            let mut attacked = false;
            for (victim, target_pos) in positions.iter() {
                if target_pos.0 == destination {
                    if player.get(victim).is_ok() {
                        attack_events.send(WantsToAttack::new(entity, victim));
                    }
                    attacked = true;
                }
            }

            if !attacked {
                move_events.send(WantsToMove::new(entity, destination));
            }
        }
    }

    // movers
    //     .iter()
    //     .filter_map(|(entity, pos, fov)| determiner.determine(entity, pos.0, fov))
    //     .for_each(|a| match a {
    //         ChaseAction::Attack(a) => attack_events.send(a),
    //         ChaseAction::Move(m) => move_events.send(m),
    //     });

    // let mut determiner = ChaseActionDeterminer::new(player.single().0, positions, map);

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
}
