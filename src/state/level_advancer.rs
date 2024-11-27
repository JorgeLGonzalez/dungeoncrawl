use crate::prelude::*;
use std::collections::HashSet;

pub fn advance_level(world: &mut World, map_builder: &MapBuilder) -> usize {
    println!("Advancing to next level");

    remove_level_entities(world);
    reset_fov(world);

    let level = set_player_on_next_level(world, map_builder);
    println!("\tAdvanced to level {}", level + 1);

    level
}

fn remove_level_entities(world: &mut World) {
    let player_entity = world
        .query_filtered::<Entity, With<Player>>()
        .iter(&world)
        .next()
        .unwrap();

    let mut to_keep: HashSet<Entity> = HashSet::from_iter(
        world
            .query::<(Entity, &Carried)>()
            .iter(&world)
            .filter(|(_, carry)| carry.0 == player_entity)
            .map(|(e, _)| e),
    );
    println!("\tRetaining {} items carried by player", to_keep.len());

    to_keep.insert(player_entity);

    let to_remove = world
        .query::<Entity>()
        .iter(&world)
        .filter_map(|e| (!to_keep.contains(&e)).then_some(e))
        .collect::<Vec<_>>();
    println!("\tRemoving {} entities", to_remove.len());
    to_remove.into_iter().for_each(|e| {
        world.despawn(e);
    });
}

fn reset_fov(world: &mut World) {
    println!("\tClearing player FOV");
    world
        .query::<&mut FieldOfView>()
        .iter_mut(world)
        .for_each(|mut fov| fov.is_dirty = true);
}

fn set_player_on_next_level(world: &mut World, map_builder: &MapBuilder) -> usize {
    if let Some((mut player, mut pos)) = world
        .query::<(&mut Player, &mut PointC)>()
        .iter_mut(world)
        .nth(0)
    {
        player.map_level += 1;
        pos.0.x = map_builder.player_start.x;
        pos.0.y = map_builder.player_start.y;

        player.map_level
    } else {
        0
    }
}
