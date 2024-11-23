use crate::prelude::*;

pub fn map_render(
    player_fov: Query<&FieldOfView, With<Player>>,
    (map, camera): (Res<Map>, Res<Camera>),
    // #[resource] theme: &Box<dyn MapTheme>,
) {
    let player_fov = player_fov.single();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(ConsoleLayer::Map.into());

    let camera_origin = Point::new(camera.left_x, camera.top_y);
    let x_range = camera.left_x..camera.right_x;

    for y in camera.top_y..=camera.bottom_y {
        for x in x_range.clone() {
            let pt = Point::new(x, y);
            if should_render(pt, player_fov, map.as_ref()) {
                draw_batch.set(
                    pt - camera_origin,
                    determine_color(&pt, &player_fov),
                    tile_to_render(map.tiles[map_idx(x, y)]), // theme.tile_to_render(map.tiles[map_idx(x, y)]),
                );
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}

fn determine_color(pt: &Point, fov: &FieldOfView) -> ColorPair {
    if fov.visible_tiles.contains(pt) {
        ColorPair::new(WHITE, BLACK)
    } else {
        ColorPair::new(DARK_GRAY, BLACK)
    }
}

fn should_render(pt: Point, fov: &FieldOfView, map: &Map) -> bool {
    let idx = map_idx(pt.x, pt.y);

    map.in_bounds(pt) && (fov.visible_tiles.contains(&pt) | map.revealed_tiles[idx])
}

fn tile_to_render(tile_type: TileType) -> u16 {
    match tile_type {
        TileType::Exit => to_cp437('>'),
        TileType::Floor => to_cp437('.'),
        TileType::Wall => to_cp437('#'),
    }
}
