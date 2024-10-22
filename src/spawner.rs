use crate::prelude::*;

pub fn spawn(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    player_pos: Point,
    amulet_pos: Point,
    rooms: &[Rect],
) {
    spawn_player(ecs, player_pos);
    spawn_amulet_of_yala(ecs, amulet_pos);

    rooms
        .iter()
        .skip(1)
        .map(|r| r.center())
        .for_each(|pos| spawn_monster(ecs, rng, pos));
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
        Enemy,
        pos,
        Render::new(ColorPair::new(WHITE, BLACK), glyph),
        ChasingPlayer {},
        Health::new(hp, hp),
        Name(name),
    ));
}

fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render::new(ColorPair::new(WHITE, BLACK), to_cp437('@')),
        Health::new(10, 10),
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}
