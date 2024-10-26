use crate::prelude::*;

pub fn spawn(ecs: &mut World, rng: &mut RandomNumberGenerator, map_builder: &MapBuilder) {
    spawn_player(ecs, map_builder.player_start);
    spawn_amulet_of_yala(ecs, map_builder.amulet_start);

    map_builder
        .monster_spawns
        .iter()
        .for_each(|pos| spawn_monster(ecs, rng, *pos));
}

fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render::new(ColorPair::new(WHITE, BLACK), to_cp437('/')),
    ));
}

fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.push((
        ChasingPlayer {},
        Enemy,
        FieldOfView::new(6),
        Health::new(hp, hp),
        Name(name),
        pos,
        Render::new(ColorPair::new(WHITE, BLACK), glyph),
    ));
}

fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        FieldOfView::new(8),
        Health::new(10, 10),
        Player,
        pos,
        Render::new(ColorPair::new(WHITE, BLACK), to_cp437('@')),
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}
