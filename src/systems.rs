mod chasing;
mod combat;
mod end_turn;
mod entity_render;
mod fov;
mod helpers;
mod hud;
mod map_render;
mod movement;
mod player_input;
mod random_move;
mod tooltips;
mod use_items;

pub use crate::prelude::*;

pub fn build_system_sets(app: &mut App) {
    app.add_system_set(
        ConditionSet::new()
            .label(StateLabel::Fov)
            .run_unless_resource_equals(TurnState::GameOver)
            .with_system(fov::fov)
            .into(),
    );

    // rendering stage part of Update stage and both systems run in parallel
    app.add_system_set(
        ConditionSet::new()
            .run_unless_resource_equals(TurnState::GameOver)
            .after(StateLabel::Fov)
            .with_system(map_render::map_render)
            .with_system(entity_render::entity_render)
            .with_system(hud::hud)
            .with_system(tooltips::tooltip)
            .into(),
    );

    app.add_system(player_input::player_input.run_if_resource_equals(TurnState::AwaitingInput));

    app.add_system_set_to_stage(
        GameStage::PlayerCombat,
        ConditionSet::new()
            .run_if_resource_equals(TurnState::PlayerTurn)
            .with_system(use_items::use_items)
            .with_system(combat::combat)
            .into(),
    );

    app.add_system_set_to_stage(
        GameStage::MovePlayer,
        ConditionSet::new()
            .run_if_resource_equals(TurnState::PlayerTurn)
            .with_system(movement::movement)
            .with_system(end_turn::end_turn)
            .into(),
    );

    app.add_system_set_to_stage(
        GameStage::PlayerFov,
        ConditionSet::new()
            .run_if_resource_equals(TurnState::PlayerTurn)
            .with_system(fov::fov)
            .into(),
    );

    app.add_system_set_to_stage(
        GameStage::GenerateMonsterMoves,
        ConditionSet::new()
            .run_if_resource_equals(TurnState::MonsterTurn)
            .with_system(random_move::random_move)
            .with_system(chasing::chasing)
            .into(),
    );

    app.add_system_set_to_stage(
        GameStage::MonsterCombat,
        ConditionSet::new()
            .run_if_resource_equals(TurnState::MonsterTurn)
            .with_system(combat::combat)
            .into(),
    );

    app.add_system_set_to_stage(
        GameStage::MoveMonsters,
        ConditionSet::new()
            .run_if_resource_equals(TurnState::MonsterTurn)
            .with_system(movement::movement)
            .with_system(end_turn::end_turn)
            .into(),
    );

    app.add_system_set_to_stage(
        GameStage::MonsterFov,
        ConditionSet::new()
            .run_if_resource_equals(TurnState::MonsterTurn)
            .with_system(fov::fov)
            .into(),
    );
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemLabel)]
enum StateLabel {
    Fov,
}
