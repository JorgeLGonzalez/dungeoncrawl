use super::helpers::player_fov;
use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let entities = gather_entities_in_render_order(ecs, camera);
    render(&entities);
}

fn render(entities: &[EntityInfo]) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(ConsoleLayer::Entity.into());

    for entity in entities.iter() {
        draw_batch.set(entity.pos, entity.render.color, entity.render.glyph);
    }

    draw_batch.submit(5000).expect("Batch error");
}

fn gather_entities_in_render_order(ecs: &SubWorld, camera: &Camera) -> Vec<EntityInfo> {
    let offset = Point::new(camera.left_x, camera.top_y);
    let player_fov = player_fov(ecs);
    let mut entities: Vec<EntityInfo> = <(&Point, &Render)>::query()
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .map(|(&pos, &render)| EntityInfo {
            pos: pos - offset,
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
