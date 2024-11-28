use crate::prelude::*;

// From our game design perspective, GameStage is a group of systems that require
// the commands in the previous group to be flushed.
// See systems.rs
#[derive(Clone, Debug, Eq, Hash, PartialEq, StageLabel)]
pub enum GameStage {
    GenerateMonsterMoves,
    MonsterCombat,
    MonsterFov,
    MovePlayer,
    MoveMonsters,
    PlayerCombat,
    PlayerFov,
}

/*
See https://saveriomiroddi.github.io/learn_bevy_ecs_by_ripping_off/07_01-02/implementing_the_states_in_bevy.html
 */
