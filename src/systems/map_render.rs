use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(ecs: &SubWorld, #[resource] map: &Map, #[resource] camera: &Camera) {
    let player_fov = player_fov(ecs);

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(ConsoleLayer::Map.into());
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            if map.in_bounds(pt) && player_fov.visible_tiles.contains(&pt) {
                draw_batch.set(
                    determine_pos(pt, camera),
                    ColorPair::new(WHITE, BLACK),
                    determine_glyph(pt, map),
                );
            }
        }
    }

    draw_batch.submit(0).expect("Batch error");
}

fn determine_glyph(Point { x, y, .. }: Point, map: &Map) -> u16 {
    let idx = map_idx(x, y);
    match map.tiles[idx] {
        TileType::Floor => to_cp437('.'),
        TileType::Wall => to_cp437('#'),
    }
}

fn determine_pos(absolute_pos: Point, camera: &Camera) -> Point {
    absolute_pos - Point::new(camera.left_x, camera.top_y)
}

fn player_fov<'a>(ecs: &'a SubWorld<'a>) -> &'a FieldOfView {
    <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap()
}
