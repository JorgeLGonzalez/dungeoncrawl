use crate::components::Name as NameComponent;
use crate::prelude::*;
// use super::helpers::player_fov;

pub fn tooltip(
    query: Query<(&PointC, &NameComponent, Option<&Health>)>,
    mouse_pos: Res<Point>,
    camera: Res<Camera>,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(ConsoleLayer::Hud.into());

    let screen_pos = *mouse_pos * 4;
    let map_pos = determine_map_pos(mouse_pos, camera);
    // let player_fov = player_fov(ecs);
    query
        .iter()
        .filter(|(pos, ..)| pos.0 == map_pos)
        .for_each(|(_, name, health)| {
            draw_batch.print(screen_pos, display(name, health));
        });

    draw_batch.submit(10100).expect("Batch error");
}

fn determine_map_pos(mouse_pos: Res<Point>, camera: Res<Camera>) -> Point {
    let offset = Point::new(camera.left_x, camera.top_y);

    *mouse_pos + offset
}

fn display(name: &NameComponent, health: Option<&Health>) -> String {
    health.map_or_else(
        || name.0.to_string(),
        |h| format!("{}: {} hp", name.0, h.current),
    )
}
