use crate::prelude::*;

pub fn movement(
    mut commands: Commands,
    mut move_events: EventReader<WantsToMove>,
    query: Query<(Entity, &FieldOfView, Option<&Player>)>,
    (mut map, mut camera): (ResMut<Map>, ResMut<Camera>),
) {
    for &WantsToMove {
        destination,
        entity,
    } in move_events.iter()
    {
        if map.can_enter_tile(destination) {
            commands.entity(entity).insert(PointC(destination));

            if let Ok((entity, fov, player)) = query.get(entity) {
                commands.entity(entity).insert(fov.clone_dirty());
                if player.is_some() {
                    camera.on_player_move(destination);
                    fov.visible_tiles.iter().for_each(|tile_pos| {
                        map.revealed_tiles[map_idx(tile_pos.x, tile_pos.y)] = true;
                    });
                }
            }
        }
    }
}

// #[system(for_each)]
// #[read_component(FieldOfView)]
// #[read_component(Player)]
// pub fn movement(
//     entity: &Entity,
//     want_move: &WantsToMove,
//     #[resource] map: &mut Map,
//     #[resource] camera: &mut Camera,
//     ecs: &mut SubWorld,
//     commands: &mut CommandBuffer,
// ) {
//     if !map.can_enter_tile(want_move.destination) {
//         return;
//     }

//     commands.add_component(want_move.entity, want_move.destination);

//     if let Some((fov, is_player)) = fov(want_move.entity, ecs) {
//         commands.add_component(want_move.entity, fov.clone_dirty());

//         if is_player {
//             handle_player_move(want_move.destination, &fov, map, camera);
//         }
//     }

//     commands.remove(*entity);
// }

// fn fov(entity: Entity, ecs: &SubWorld) -> Option<(FieldOfView, bool)> {
//     if let Ok(entry) = ecs.entry_ref(entity) {
//         if let Ok(fov) = entry.get_component::<FieldOfView>() {
//             let is_player = is_player(&entry);

//             return Some((fov.clone(), is_player));
//         }
//     }

//     None
// }

// fn handle_player_move(destination: Point, fov: &FieldOfView, map: &mut Map, camera: &mut Camera) {
//     fov.visible_tiles.iter().for_each(|pos| {
//         map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
//     });

//     camera.on_player_move(destination);
// }

// fn is_player(entry: &EntryRef<'_>) -> bool {
//     entry.get_component::<Player>().is_ok()
// }
