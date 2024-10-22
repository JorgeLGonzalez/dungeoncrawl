use crate::{prelude::*, turn_state};

#[system]
#[read_component(AmuletOfYala)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    if *turn_state == TurnState::AwaitingInput {
        return;
    }

    let new_state = if player_died(ecs) {
        TurnState::GameOver
    } else if amulet_hit(ecs) {
        TurnState::Victory
    } else {
        match *turn_state {
            TurnState::MonsterTurn => TurnState::AwaitingInput,
            TurnState::PlayerTurn => TurnState::MonsterTurn,
            _ => turn_state.clone(),
        }
    };

    *turn_state = new_state;
}

fn amulet_hit(ecs: &SubWorld) -> bool {
    let amulet_pos = <&Point>::query()
        .filter(component::<AmuletOfYala>())
        .iter(ecs)
        .nth(0)
        .unwrap();
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
