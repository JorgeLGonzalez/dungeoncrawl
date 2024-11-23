use crate::prelude::*;

pub fn movement(
    mut commands: Commands,
    mut move_events: EventReader<WantsToMove>,
    query: Query<(Entity, &FieldOfView, Option<&Player>)>,
    (mut map, mut camera): (ResMut<Map>, ResMut<Camera>),
) {
    let valid_moves: Vec<&WantsToMove> = move_events
        .iter()
        .filter(|m| map.can_enter_tile(m.destination))
        .collect();

    for &WantsToMove {
        destination,
        entity,
    } in valid_moves
    {
        commands.entity(entity).insert(PointC(destination));

        if let Ok((entity, fov, player)) = query.get(entity) {
            commands.entity(entity).insert(fov.clone_dirty());
            if player.is_some() {
                handle_player_move(destination, fov, map.as_mut(), camera.as_mut());
            }
        }
    }
}

fn handle_player_move(destination: Point, fov: &FieldOfView, map: &mut Map, camera: &mut Camera) {
    fov.visible_tiles.iter().for_each(|pos| {
        map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
    });

    camera.on_player_move(destination);
}
