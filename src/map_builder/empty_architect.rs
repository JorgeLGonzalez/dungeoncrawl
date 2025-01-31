use super::MapArchitect;
use crate::prelude::*;

pub struct EmptyArchitect;

impl MapArchitect for EmptyArchitect {
    fn name(&self) -> String {
        "EmptyArchitect".to_string()
    }

    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::create(TileType::Floor);
        mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        mb.amulet_start = mb.find_farthest();

        for _ in 0..50 {
            mb.monster_spawns.push(Point::new(
                rng.range(1, SCREEN_WIDTH),
                rng.range(1, SCREEN_HEIGHT),
            ))
        }

        mb
    }
}
