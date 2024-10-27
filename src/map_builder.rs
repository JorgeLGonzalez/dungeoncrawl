mod cell_automata_architect;
mod drunkard_walk_architect;
mod empty_architect;
mod map_distance;
mod prefab_vault;
mod prefabs;
mod rooms_architect;

use crate::prelude::*;
use cell_automata_architect::CellAutomataArchitect;
use drunkard_walk_architect::DrunkardsWalkArchitect;
use map_distance::MapDistance;
use prefab_vault::PrefabVault;
use rooms_architect::RoomsArchitect;

const NUM_ROOMS: usize = 20;

trait MapArchitect {
    fn name(&self) -> String;
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub struct MapBuilder {
    pub amulet_start: Point,
    pub map: Map,
    pub monster_spawns: Vec<Point>,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 3) {
            0 => Box::new(DrunkardsWalkArchitect),
            1 => Box::new(RoomsArchitect),
            _ => Box::new(CellAutomataArchitect),
        };

        println!("Building map using {}", architect.name());

        let mut mb = architect.new(rng);
        PrefabVault::new(prefabs::FORTRESS, rng).apply(&mut mb);

        mb
    }

    fn create(fill: TileType) -> Self {
        let mut mb = Self {
            amulet_start: Point::zero(),
            map: Map::new(),
            monster_spawns: Vec::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };
        mb.fill(fill);

        mb
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                self.tunnel_horizontally(prev.x, new.x, prev.y);
                self.tunnel_vertically(prev.y, new.y, new.x);
            } else {
                self.tunnel_vertically(prev.y, new.y, prev.x);
                self.tunnel_horizontally(prev.x, new.x, new.y);
            }
        }
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room);
            }
        }
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_farthest(&self) -> Point {
        MapDistance::new(&self.map, self.player_start).find_farthest()
    }

    fn spawn_monsters(&self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        let mut spawnable_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| {
                **t == TileType::Floor
                    && DistanceAlg::Pythagoras.distance2d(*start, self.map.index_to_point2d(*idx))
                        > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();

        const NUM_MONSTERS: usize = 50;
        let mut spawns = Vec::new();
        for _ in 0..NUM_MONSTERS {
            let target_index = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles.remove(target_index));
        }

        spawnable_tiles
    }

    fn tunnel_horizontally(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn tunnel_vertically(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
}
