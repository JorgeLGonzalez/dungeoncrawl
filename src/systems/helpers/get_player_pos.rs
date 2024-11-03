use crate::prelude::*;

pub fn get_player_pos(ecs: &SubWorld) -> Point {
    let mut player = <(&Point, &Player)>::query();
    *player.iter(ecs).nth(0).unwrap().0
}
