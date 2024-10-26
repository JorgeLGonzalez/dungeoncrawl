use crate::prelude::*;

#[system(for_each)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if !map.can_enter_tile(want_move.destination) {
        return;
    }

    commands.add_component(want_move.entity, want_move.destination);

    if ecs
        .entry_ref(want_move.entity)
        .unwrap()
        .get_component::<Player>()
        .is_ok()
    {
        camera.on_player_move(want_move.destination);
    }

    if let Ok(entry) = ecs.entry_ref(want_move.entity) {
        if let Ok(fov) = entry.get_component::<FieldOfView>() {
            commands.add_component(want_move.entity, fov.clone_dirty());
        }
    }

    commands.remove(*entity);
}
