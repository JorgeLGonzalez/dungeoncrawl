use crate::prelude::*;

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let glyph = match rng.range(0, 4) {
        0 => to_cp437('E'),
        1 => to_cp437('O'),
        2 => to_cp437('o'),
        _ => to_cp437('g'),
    };
    ecs.push((
        Enemy,
        pos,
        Render::new(ColorPair::new(WHITE, BLACK), glyph),
        MovingRandomly {},
    ));
}

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render::new(ColorPair::new(WHITE, BLACK), to_cp437('@')),
    ));
}
