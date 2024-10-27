use crate::prelude::*;

use super::map_distance::MapDistance;

pub struct PrefabVault<'a> {
    blueprint: String,
    height: i32,
    rng: &'a mut RandomNumberGenerator,
    width: i32,
}

impl<'a> PrefabVault<'a> {
    pub fn new(blueprint: &str, rng: &'a mut RandomNumberGenerator) -> Self {
        Self {
            blueprint: blueprint.to_string(),
            height: 11,
            rng,
            width: 12,
        }
    }

    pub fn apply(&mut self, mb: &mut MapBuilder) {
        if let Some(placement) = self.try_place(mb) {
            let blueprint: Vec<char> = self.trim_newlines();
            let mut blueprint_idx = 0;
            for ty in placement.y..placement.y + self.height {
                for tx in placement.x..placement.x + self.width {
                    set_tile(blueprint[blueprint_idx], Point::new(tx, ty), mb);
                    blueprint_idx += 1;
                }
            }
        } else {
            println!("*** Unable to place prefab vault");
        }
    }

    fn create_vault(&mut self) -> Rect {
        Rect::with_size(
            self.rng.range(0, SCREEN_WIDTH - self.width),
            self.rng.range(0, SCREEN_HEIGHT - self.height),
            self.width,
            self.height,
        )
    }

    fn trim_newlines(&self) -> Vec<char> {
        self.blueprint
            .chars()
            .filter(|a| *a != '\r' && *a != '\n')
            .collect()
    }

    fn try_place(&mut self, mb: &mut MapBuilder) -> Option<Point> {
        let mut placement = None;
        let dijkstra_map = MapDistance::new(&mb.map, mb.player_start).create_dijkstra_map();

        let mut attempts = 0;
        while placement.is_none() && attempts < 10 {
            let vault = self.create_vault();

            if can_place(&vault, &dijkstra_map, mb) {
                placement = Some(Point::new(vault.x1, vault.y1));
                clear_monsters(&vault, mb);
                log_placement(&vault, attempts);
            }

            attempts += 1;
        }

        placement
    }
}

fn can_place(vault: &Rect, dijkstra_map: &DijkstraMap, mb: &MapBuilder) -> bool {
    let mut can_place = false;
    vault.for_each(|pt| {
        let idx = mb.map.point2d_to_index(pt);
        let distance = dijkstra_map.map[idx];
        if distance < 2000.0 && distance > 20.0 && mb.amulet_start != pt {
            can_place = true;
        }
    });

    can_place
}

fn clear_monsters(vault: &Rect, mb: &mut MapBuilder) {
    let points = vault.point_set();
    mb.monster_spawns.retain(|pt| !points.contains(pt));
}

fn log_placement(vault: &Rect, attempts: i32) {
    println!(
        "Placing {}x{} (area={}) prefab vault at ({},{}) on attempt {attempts}",
        vault.width(),
        vault.height(),
        vault.height() * vault.width(),
        vault.x1,
        vault.y1,
    );
}

fn set_tile(blueprint_symbol: char, pos: Point, mb: &mut MapBuilder) {
    let idx = mb.map.point2d_to_index(pos);
    match blueprint_symbol {
        'M' => {
            mb.map.tiles[idx] = TileType::Floor;
            mb.monster_spawns.push(pos);
        }
        '-' => mb.map.tiles[idx] = TileType::Floor,
        '#' => mb.map.tiles[idx] = TileType::Wall,
        _ => println!("No idea what to do with [{blueprint_symbol}]"),
    }
}
