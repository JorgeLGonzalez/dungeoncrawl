use super::helpers::mover::prelude::*;
use crate::prelude::*;

pub fn movement(
    mut camera: ResMut<MainCamera>,
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut move_events: EventReader<WantsToMove>,
    fov_query: FovQuery,
    turn: Res<TurnState>,
) {
    move_events
        .iter()
        .map(|m| Mover::new(&fov_query, m))
        .filter(|m| !m.out_of_turn(turn.to_owned()))
        .filter(|m| map.can_enter_tile(m.destination))
        .collect::<Vec<_>>()
        .iter()
        .for_each(|mover| {
            mover.do_move(&mut commands);
            mover.handle_player_move(&mut camera, &mut map);
        });
}
