use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        _ => turn_state.clone(),
    };

    *turn_state = if player_died(ecs) {
        TurnState::GameOver
    } else {
        new_state
    };
}

fn player_died(ecs: &SubWorld) -> bool {
    <&Health>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .find(|hp| hp.current < 1)
        .is_some()
}
