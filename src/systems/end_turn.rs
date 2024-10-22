use crate::prelude::*;

#[system]
#[read_component(AmuletOfYala)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
    let amulet_pos = amulet.iter(ecs).nth(0).unwrap();
    let player_pos = <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();

    let new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        _ => turn_state.clone(),
    };

    *turn_state = if player_died(ecs) {
        TurnState::GameOver
    } else if player_pos == amulet_pos {
        TurnState::Victory
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
