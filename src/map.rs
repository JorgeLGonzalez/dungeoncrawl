use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Exit,
    Floor,
    Wall,
}

pub struct Map {
    pub revealed_tiles: Vec<bool>,
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            revealed_tiles: vec![false; NUM_TILES],
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && is_entry_tile(self.tiles[map_idx(point.x, point.y)])
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if !self.in_bounds(destination) {
            return None;
        }

        if self.can_enter_tile(destination) {
            let idx = self.point2d_to_index(destination);
            Some(idx)
        } else {
            None
        }
    }
}

fn is_entry_tile(tile_type: TileType) -> bool {
    const ENTRY_TILES: [TileType; 2] = [TileType::Floor, TileType::Exit];

    ENTRY_TILES.iter().find(|&&t| t == tile_type).is_some()
}

/// Convert x,y to vector index. Called striding. See pp 162-163
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        self.in_bounds(pos)
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let location = self.index_to_point2d(idx);

        [
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, -1),
            Point::new(0, 1),
        ]
        .iter()
        .filter_map(|delta| self.valid_exit(location, *delta))
        .map(|idx| (idx, 1.0))
        .collect()
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }

    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] != TileType::Floor
    }
}
