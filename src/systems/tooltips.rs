use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Name)]
#[read_component(Point)]
pub fn tooltip(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    // TODO why are positions mut
    let mut positions = <(Entity, &Point, &Name)>::query();
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(3);

    positions
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos)
        .for_each(|(entity, _, name)| {
            let screen_pos = *mouse_pos * 4; // adjust to x-layer dims p. 278
            let display =
                if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                    format!("{}: {} hp", &name.0, health.current)
                } else {
                    name.0.clone()
                };
            draw_batch.print(screen_pos, &display);
        });

    draw_batch.submit(10100).expect("Batch error");
}
