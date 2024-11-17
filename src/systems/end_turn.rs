use crate::prelude::*;

pub fn end_turn(mut commands: Commands, turn_state: Res<TurnState>) {
    let new_state = match *turn_state {
        TurnState::AwaitingInput => unreachable!(),
        TurnState::GameOver => todo!(),
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        TurnState::NextLevel => todo!(),
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::Victory => todo!(),
    };

    commands.insert_resource(new_state);

    // let player_pos = get_player_pos(ecs);

    // let new_state = if player_died(ecs) {
    //     TurnState::GameOver
    // } else if player_pos == amulet_pos(ecs) {
    //     TurnState::Victory
    // } else if player_tile(player_pos, map) == TileType::Exit {
    //     TurnState::NextLevel
    // } else {
    //     match *turn_state {
    //         TurnState::MonsterTurn => TurnState::AwaitingInput,
    //         TurnState::PlayerTurn => TurnState::MonsterTurn,
    //         _ => turn_state.clone(),
    //     }
    // };

    // *turn_state = new_state;
}

// fn amulet_pos(ecs: &SubWorld) -> Point {
//     <&Point>::query()
//         .filter(component::<AmuletOfYala>())
//         .iter(ecs)
//         .nth(0)
//         .map_or_else(|| Point::new(-1, -1), |&p| p)
// }

// fn player_died(ecs: &SubWorld) -> bool {
//     <&Health>::query()
//         .filter(component::<Player>())
//         .iter(ecs)
//         .find(|hp| hp.current < 1)
//         .is_some()
// }

// fn player_tile(player_pos: Point, map: &Map) -> TileType {
//     map.tiles[map.point2d_to_index(player_pos)]
// }
