use crate::prelude::*;

pub fn entity_render(
    query: Query<(&PointC, &Render)>,
    player_fov: Query<&FieldOfView, With<Player>>,
    camera: Res<Camera>,
) {
    let player_fov = player_fov.single();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(ConsoleLayer::Entity.into());

    let offset = Point::new(camera.left_x, camera.top_y);

    for (pos, render) in query
        .iter()
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos.0))
    {
        draw_batch.set(pos.0 - offset, render.color, render.glyph);
    }

    draw_batch.submit(5000).expect("Batch error");
}

fn gather_entities_in_render_order(
    query: Query<(&PointC, &Render)>,
    camera: Res<Camera>,
) -> Vec<EntityInfo> {
    let offset = Point::new(camera.left_x, camera.top_y);
    // let player_fov = player_fov(ecs);
    let mut entities: Vec<EntityInfo> = query
        .iter()
        // .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .map(|(pos, &render)| EntityInfo {
            pos: pos.0 - offset,
            render,
        })
        .collect();
    entities.sort_by_key(|e| e.render.order);

    entities
}

struct EntityInfo {
    pos: Point,
    render: Render,
}
