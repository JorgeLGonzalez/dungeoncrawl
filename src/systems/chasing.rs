use crate::prelude::*;

#[system]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &ChasingPlayer)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    let player_pos = player.iter(ecs).nth(0).unwrap().0;
    let player_idx = map_idx(player_pos.x, player_pos.y);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    movers.iter(ecs).for_each(|(entity, pos, _)| {
        let idx = map_idx(pos.x, pos.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
            // see p 315 for rationale for 1.2
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                *player_pos
            };

            let mut attacked = false;
            positions
                .iter(ecs)
                .filter(|(_, target_pos, _)| **target_pos == destination)
                .for_each(|(victim, ..)| {
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
        }
    });
}
