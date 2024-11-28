use crate::prelude::*;

pub fn entity_render(
    entities: Query<(&PointC, &Render)>,
    player_fov: Query<&FieldOfView, With<Player>>,
    camera: Res<MainCamera>,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(ConsoleLayer::Entity.into());

    gather_entities_in_render_order(entities, player_fov, camera)
        .iter()
        .for_each(|entity| {
            draw_batch.set(entity.pos, entity.render.color, entity.render.glyph);
        });

    draw_batch.submit(5000).expect("Batch error");
}

/// Return info only for entities visible to the player.
/// Return in render order so that monsters and player are always rendered on
/// top of items etc.
fn gather_entities_in_render_order(
    entities_query: Query<(&PointC, &Render)>,
    player_fov_query: Query<&FieldOfView, With<Player>>,
    camera: Res<MainCamera>,
) -> Vec<EntityInfo> {
    let offset = Point::new(camera.left_x, camera.top_y);
    let player_fov = player_fov_query.single();

    let mut entities: Vec<EntityInfo> = entities_query
        .iter()
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos.0))
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
