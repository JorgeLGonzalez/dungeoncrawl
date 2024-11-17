use crate::prelude::*;

pub fn collisions(
    mut commands: Commands,
    player_query: Query<&PointC, With<Player>>,
    enemies_query: Query<(Entity, &PointC), With<Enemy>>,
) {
    let player_pos = player_query.single().0;

    for (entity, pos) in enemies_query.iter() {
        if pos.0 == player_pos {
            println!("Collision at {:?}", player_pos);
            commands.entity(entity).despawn();
        }
    }
}
