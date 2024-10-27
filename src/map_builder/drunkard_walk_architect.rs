use super::{map_distance::MapDistance, MapArchitect};
use crate::prelude::*;

const DESIRED_FLOOR: usize = NUM_TILES / 3;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const STAGGER_DISTANCE: usize = 400;

pub struct DrunkardsWalkArchitect;

impl DrunkardsWalkArchitect {
    fn drunkard(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut drunkard_pos = start.clone();
        let mut distance_staggered = 0;

        while distance_staggered <= STAGGER_DISTANCE {
            to_floor(drunkard_pos, map);

            step(&mut drunkard_pos, rng);
            if !map.in_bounds(drunkard_pos) {
                break;
            }

            distance_staggered += 1;
        }
    }

    fn tunnel(&mut self, mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
        self.drunkard(&mb.player_start, rng, &mut mb.map);

        while insufficient_floor(&mb) {
            self.drunkard(
                &Point::new(rng.range(0, SCREEN_WIDTH), rng.range(0, SCREEN_HEIGHT)),
                rng,
                &mut mb.map,
            );
            MapDistance::new(&mb.map, mb.player_start)
                .enum_dijkstra()
                .filter(|l| l.distance > 2000.0)
                .for_each(|l| mb.map.tiles[l.pos_idx] = TileType::Wall);
        }
    }
}

impl MapArchitect for DrunkardsWalkArchitect {
    fn name(&self) -> String {
        "DrunkardsWalkArchitect".to_string()
    }

    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::create(TileType::Wall);

        mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        self.tunnel(&mut mb, rng);

        mb.monster_spawns = mb.spawn_monsters(&mb.player_start, rng);
        mb.amulet_start = mb.find_farthest();

        mb
    }
}

fn insufficient_floor(mb: &MapBuilder) -> bool {
    mb.map
        .tiles
        .iter()
        .filter(|t| **t == TileType::Floor)
        .count()
        < DESIRED_FLOOR
}

fn to_floor(pos: Point, map: &mut Map) {
    let idx = map.point2d_to_index(pos);
    map.tiles[idx] = TileType::Floor;
}

fn step(pos: &mut Point, rng: &mut RandomNumberGenerator) {
    match rng.range(0, 4) {
        0 => pos.x -= 1,
        1 => pos.x += 1,
        2 => pos.y -= 1,
        _ => pos.y += 1,
    }
}
