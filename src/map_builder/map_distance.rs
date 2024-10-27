use crate::prelude::*;
use std::cmp::Ordering;

pub struct MapDistance<'a> {
    map: &'a Map,
    origin: Point,
}

impl<'a> MapDistance<'a> {
    pub fn new(map: &'a Map, origin: Point) -> Self {
        Self { map, origin }
    }

    pub fn create_dijkstra_map(&self) -> DijkstraMap {
        DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.map.point2d_to_index(self.origin)],
            self.map,
            1024.0,
        )
    }

    pub fn find_farthest(&self) -> Point {
        let farthest_idx = self.enum_dijkstra().max_by(distance).unwrap().pos_idx;

        self.map.index_to_point2d(farthest_idx)
    }

    pub fn enum_dijkstra(&self) -> impl Iterator<Item = DijkstraLocation> {
        let dijkstra_map = self.create_dijkstra_map();

        const UNREACHABLE: f32 = f32::MAX;
        dijkstra_map
            .map
            .into_iter()
            .enumerate()
            .filter(|(_, dist)| *dist < UNREACHABLE)
            .map(DijkstraLocation::from_tuple)
    }
}

fn distance(a: &DijkstraLocation, b: &DijkstraLocation) -> Ordering {
    a.distance.partial_cmp(&b.distance).unwrap()
}

pub struct DijkstraLocation {
    pub distance: f32,
    pub pos_idx: usize,
}

impl DijkstraLocation {
    pub fn from_tuple((pos_idx, distance): (usize, f32)) -> Self {
        Self { distance, pos_idx }
    }
}
