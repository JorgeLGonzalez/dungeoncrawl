use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Name)]
#[read_component(Point)]
pub fn tooltip(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(3);

    let map_pos = determine_map_pos(*mouse_pos, camera);
    <(Entity, &Point, &Name)>::query()
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos)
        .for_each(|(entity, _, name)| {
            let screen_pos = *mouse_pos * 4;
            draw_batch.print(screen_pos, display(ecs, *entity, &name.0));
        });

    draw_batch.submit(10100).expect("Batch error");
}

fn determine_map_pos(mouse_pos: Point, camera: &Camera) -> Point {
    let offset = Point::new(camera.left_x, camera.top_y);

    mouse_pos + offset
}

fn display(ecs: &SubWorld, entity: Entity, name: &str) -> String {
    if let Ok(health) = ecs.entry_ref(entity).unwrap().get_component::<Health>() {
        format!("{}: {} hp", name, health.current)
    } else {
        name.to_string()
    }
}
