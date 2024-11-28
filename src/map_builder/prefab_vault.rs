use super::map_distance::MapDistance;
use crate::prelude::*;

pub struct PrefabVault<'a> {
    blueprint: Vec<char>,
    dijkstra_map: DijkstraMap,
    height: i32,
    mb: &'a mut MapBuilder,
    rng: &'a mut RandomNumberGenerator,
    width: i32,
}

impl<'a> PrefabVault<'a> {
    pub fn apply(blueprint: &str, mb: &'a mut MapBuilder, rng: &'a mut RandomNumberGenerator) {
        let blueprint = blueprint
            .chars()
            .filter(|a| *a != '\r' && *a != '\n')
            .collect();
        let dijkstra_map = MapDistance::new(&mb.map, mb.player_start).create_dijkstra_map();

        let mut placer = Self {
            blueprint,
            dijkstra_map,
            height: 11,
            mb,
            rng,
            width: 12,
        };

        if let Some(placement) = placer.find_placement() {
            placer.map_vault(placement);
        } else {
            println!("*** Unable to place prefab vault");
        }
    }

    fn can_place(&self, vault: &BracketRect) -> bool {
        let mut can_place = false;
        vault.for_each(|pt| {
            let idx = self.mb.map.point2d_to_index(pt);
            let distance = self.dijkstra_map.map[idx];

            if distance < 2000.0 && distance > 20.0 && self.mb.amulet_start != pt {
                can_place = true;
            }
        });

        can_place
    }

    fn clear_monsters(&mut self, vault: &BracketRect) {
        let points = vault.point_set();
        self.mb.monster_spawns.retain(|pt| !points.contains(pt));
    }

    fn create_vault(&mut self) -> BracketRect {
        BracketRect::with_size(
            self.rng.range(0, SCREEN_WIDTH - self.width),
            self.rng.range(0, SCREEN_HEIGHT - self.height),
            self.width,
            self.height,
        )
    }

    fn find_placement(&mut self) -> Option<Point> {
        let mut placement = None;

        let mut attempts = 0;
        while placement.is_none() && attempts < 10 {
            let vault = self.create_vault();

            if self.can_place(&vault) {
                placement = Some(Point::new(vault.x1, vault.y1));
                self.clear_monsters(&vault);
                log_placement(&vault, attempts);
            }

            attempts += 1;
        }

        placement
    }

    fn map_vault(&mut self, placement: Point) {
        let mut blueprint_idx = 0;
        for ty in placement.y..placement.y + self.height {
            for tx in placement.x..placement.x + self.width {
                self.set_tile(blueprint_idx, Point::new(tx, ty));
                blueprint_idx += 1;
            }
        }
    }

    fn set_tile(&mut self, blueprint_idx: usize, pos: Point) {
        let idx = self.mb.map.point2d_to_index(pos);
        match self.blueprint[blueprint_idx] {
            'M' => {
                self.mb.map.tiles[idx] = TileType::Floor;
                self.mb.monster_spawns.push(pos);
            }
            '-' => self.mb.map.tiles[idx] = TileType::Floor,
            '#' => self.mb.map.tiles[idx] = TileType::Wall,
            _ => println!("No idea what to do with [{blueprint_idx}]"),
        }
    }
}

fn log_placement(vault: &BracketRect, attempts: i32) {
    println!(
        "Placing {}x{} (area={}) prefab vault at ({},{}) on attempt {attempts}",
        vault.width(),
        vault.height(),
        vault.height() * vault.width(),
        vault.x1,
        vault.y1,
    );
}
