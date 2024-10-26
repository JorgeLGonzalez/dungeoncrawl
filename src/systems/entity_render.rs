use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(ConsoleLayer::Entity.into());
    let offset = Point::new(camera.left_x, camera.top_y);

    let player_fov = player_fov(ecs);
    <(&Point, &Render)>::query()
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Batch error");
}

fn player_fov<'a>(ecs: &'a SubWorld<'a>) -> &'a FieldOfView {
    <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap()
}
