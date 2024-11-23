mod chasing;
mod collisions;
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

    // TODO: player combat stage

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

    // TODO remove
    app.add_system_set_to_stage(
        GameStage::Collisions,
        ConditionSet::new()
            .run_if_resource_equals(TurnState::PlayerTurn)
            .with_system(collisions::collisions)
            .with_system(fov::fov)
            .with_system(end_turn::end_turn)
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

    // TODO: player combat stage

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

/*
See https://saveriomiroddi.github.io/learn_bevy_ecs_by_ripping_off/07_01-02/implementing_the_states_in_bevy.html
 */

// pub fn build_input_scheduler() -> Schedule {
//     Schedule::builder()
//         .add_system(player_input::player_input_system())
//         .add_system(fov::fov_system())
//         .flush()
//         .add_system(map_render::map_render_system())
//         .add_system(entity_render::entity_render_system())
//         .add_system(hud::hud_system())
//         .add_system(tooltips::tooltip_system())
//         .build()
// }
// pub fn build_monster_scheduler() -> Schedule {
//     Schedule::builder()
//         .add_system(use_items::use_items_system())
//         .add_system(random_move::random_move_system())
//         .add_system(chasing::chasing_system())
//         .flush()
//         .add_system(combat::combat_system())
//         .flush()
//         .add_system(movement::movement_system())
//         .flush()
//         .add_system(fov::fov_system())
//         .flush()
//         .add_system(map_render::map_render_system())
//         .add_system(entity_render::entity_render_system())
//         .add_system(hud::hud_system())
//         .add_system(end_turn::end_turn_system())
//         .build()
// }

// pub fn build_player_scheduler() -> Schedule {
//     Schedule::builder()
//         .add_system(use_items::use_items_system())
//         .add_system(combat::combat_system())
//         .flush()
//         .add_system(movement::movement_system())
//         .flush()
//         .add_system(fov::fov_system())
//         .flush()
//         .add_system(map_render::map_render_system())
//         .add_system(entity_render::entity_render_system())
//         .add_system(hud::hud_system())
//         .add_system(end_turn::end_turn_system())
//         .build()
// }
