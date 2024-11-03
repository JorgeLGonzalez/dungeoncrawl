use std::collections::HashSet;

use crate::prelude::*;

pub fn advance_level(ecs: &mut World, map_builder: &MapBuilder) -> u32 {
    remove_level_entities(ecs);
    reset_fov(ecs);

    set_player_on_next_level(ecs, map_builder)
}

fn remove_level_entities(ecs: &mut World) {
    let player_entity = *<Entity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();

    let mut entities_to_keep = HashSet::new();
    entities_to_keep.insert(player_entity);

    <(Entity, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, carry)| carry.0 == player_entity)
        .map(|(e, _carry)| *e)
        .for_each(|e| {
            entities_to_keep.insert(e);
        });

    let mut cb = CommandBuffer::new(ecs);
    for e in Entity::query().iter(ecs) {
        if !entities_to_keep.contains(e) {
            cb.remove(*e);
        }
    }
    cb.flush(ecs);
}

fn reset_fov(ecs: &mut World) {
    <&mut FieldOfView>::query().iter_mut(ecs).for_each(|fov| {
        fov.is_dirty = true;
    });
}

fn set_player_on_next_level(ecs: &mut World, map_builder: &MapBuilder) -> u32 {
    let mut map_level = 0;
    <(&mut Player, &mut Point)>::query()
        .iter_mut(ecs)
        .for_each(|(player, pos)| {
            player.map_level += 1;
            map_level = player.map_level;
            pos.x = map_builder.player_start.x;
            pos.y = map_builder.player_start.y;
        });

    map_level
}
