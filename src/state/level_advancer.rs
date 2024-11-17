use std::collections::HashSet;

use crate::prelude::*;

pub fn advance_level(ecs: &mut World, map_builder: &MapBuilder) -> usize {
    remove_level_entities(ecs);
    reset_fov(ecs);

    set_player_on_next_level(ecs, map_builder)
}

fn remove_level_entities(ecs: &mut World) {
    // let mut to_keep = HashSet::new();

    // let player_entity = *<Entity>::query()
    //     .filter(component::<Player>())
    //     .iter(ecs)
    //     .nth(0)
    //     .unwrap();
    // to_keep.insert(player_entity);

    // to_keep.extend(
    //     <(Entity, &Carried)>::query()
    //         .iter(ecs)
    //         .filter(|(_, carry)| carry.0 == player_entity)
    //         .map(|(e, _carry)| *e),
    // );

    // let mut cb = CommandBuffer::new(ecs);
    // for e in Entity::query().iter(ecs) {
    //     if !to_keep.contains(e) {
    //         cb.remove(*e);
    //     }
    // }
    // cb.flush(ecs);
}

fn reset_fov(ecs: &mut World) {
    // <&mut FieldOfView>::query().iter_mut(ecs).for_each(|fov| {
    //     fov.is_dirty = true;
    // });
}

fn set_player_on_next_level(ecs: &mut World, map_builder: &MapBuilder) -> usize {
    let mut map_level: usize = 0;
    // <(&mut Player, &mut Point)>::query()
    //     .iter_mut(ecs)
    //     .for_each(|(player, pos)| {
    //         player.map_level += 1;
    //         map_level = player.map_level;
    //         pos.x = map_builder.player_start.x;
    //         pos.y = map_builder.player_start.y;
    //     });

    map_level
}
