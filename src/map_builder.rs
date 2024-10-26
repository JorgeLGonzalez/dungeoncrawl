mod empty_architect;
mod rooms_architect;

use crate::prelude::*;
use rooms_architect::RoomsArchitect;
use std::cmp::Ordering;

const NUM_ROOMS: usize = 20;

trait MapArchitect {
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
        let mut architect = RoomsArchitect;

        architect.new(rng)
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

    fn create_dijkstra_map(&self, player_pos: Point) -> DijkstraMap {
        DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.map.point2d_to_index(player_pos)],
            &self.map,
            1024.0,
        )
    }

    fn enum_dijkstra(&self, player_pos: Point) -> impl Iterator<Item = DijkstraLocation> {
        let dijkstra_map = self.create_dijkstra_map(player_pos);

        const UNREACHABLE: f32 = f32::MAX;
        dijkstra_map
            .map
            .into_iter()
            .enumerate()
            .filter(|(_, dist)| *dist < UNREACHABLE)
            .map(DijkstraLocation::from_tuple)
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_farthest(&self) -> Point {
        let farthest_idx = self
            .enum_dijkstra(self.player_start)
            .max_by(distance)
            .unwrap()
            .pos_idx;

        self.map.index_to_point2d(farthest_idx)
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

fn distance(a: &DijkstraLocation, b: &DijkstraLocation) -> Ordering {
    a.distance.partial_cmp(&b.distance).unwrap()
}

struct DijkstraLocation {
    pub distance: f32,
    pub pos_idx: usize,
}

impl DijkstraLocation {
    pub fn from_tuple((pos_idx, distance): (usize, f32)) -> Self {
        Self { distance, pos_idx }
    }
}
