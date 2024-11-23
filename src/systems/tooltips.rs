use crate::components::Name as NameComponent;
use crate::prelude::*;

pub fn tooltip(
    query: Query<(&PointC, &NameComponent, Option<&Health>)>,
    player_fov: Query<&FieldOfView, With<Player>>,
    mouse_pos: Res<Point>,
    camera: Res<Camera>,
) {
    let player_fov = player_fov.single();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(ConsoleLayer::Hud.into());

    let screen_pos = *mouse_pos * 4;
    let map_pos = determine_map_pos(mouse_pos, camera);
    query
        .iter()
        .filter(|(pos, ..)| pos.0 == map_pos && player_fov.visible_tiles.contains(&pos.0))
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
