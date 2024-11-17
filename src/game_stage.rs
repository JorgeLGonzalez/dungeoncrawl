use crate::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq, StageLabel)]
pub enum GameStage {
    Collisions,
    MovePlayer,
    MoveMonsters,
}

/*
See https://saveriomiroddi.github.io/learn_bevy_ecs_by_ripping_off/07_01-02/implementing_the_states_in_bevy.html
 */
