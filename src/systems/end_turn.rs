use crate::prelude::*;

#[system]
#[read_component(AmuletOfYala)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState, #[resource] map: &Map) {
    if *turn_state == TurnState::AwaitingInput {
        return;
    }

    let new_state = if player_died(ecs) {
        TurnState::GameOver
    } else if amulet_hit(ecs) {
        TurnState::Victory
    } else {
        let player_pos = <&Point>::query()
            .filter(component::<Player>())
            .iter(ecs)
            .nth(0)
            .unwrap();

        let idx = map.point2d_to_index(*player_pos);
        if map.tiles[idx] == TileType::Exit {
            TurnState::NextLevel
        } else {
            match *turn_state {
                TurnState::MonsterTurn => TurnState::AwaitingInput,
                TurnState::PlayerTurn => TurnState::MonsterTurn,
                _ => turn_state.clone(),
            }
        }
    };

    *turn_state = new_state;
}

fn amulet_hit(ecs: &SubWorld) -> bool {
    let amulet_default = Point::new(-1, -1);
    let amulet_pos = <&Point>::query()
        .filter(component::<AmuletOfYala>())
        .iter(ecs)
        .nth(0)
        .unwrap_or(&amulet_default);

    let player_pos = <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();

    player_pos == amulet_pos
}

fn player_died(ecs: &SubWorld) -> bool {
    <&Health>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .find(|hp| hp.current < 1)
        .is_some()
}
