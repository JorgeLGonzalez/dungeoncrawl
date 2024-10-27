use crate::prelude::*;

pub const FORTRESS: &str = "
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------
";

pub struct PrefabVault {
    blueprint: String,
    height: i32,
    width: i32,
}

impl PrefabVault {
    pub fn new(blueprint: &str) -> Self {
        Self {
            blueprint: blueprint.to_string(),
            height: 11,
            width: 12,
        }
    }

    pub fn apply(&self, mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
        let mut placement = None;
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![mb.map.point2d_to_index(mb.player_start)],
            &mb.map,
            1024.0,
        );

        let mut attempts = 0;
        while placement.is_none() && attempts < 10 {
            let dimensions = Rect::with_size(
                rng.range(0, SCREEN_WIDTH - self.width),
                rng.range(0, SCREEN_HEIGHT - self.height),
                self.width,
                self.height,
            );
            let mut can_place = false;
            dimensions.for_each(|pt| {
                let idx = mb.map.point2d_to_index(pt);
                let distance = dijkstra_map.map[idx];
                if distance < 2000.0 && distance > 20.0 && mb.amulet_start != pt {
                    can_place = true;
                }
            });

            if can_place {
                println!(
                    "Placing {}x{} (area={}) prefab vault at ({},{}) on attempt {attempts}",
                    dimensions.width(),
                    dimensions.height(),
                    dimensions.height() * dimensions.width(),
                    dimensions.x1,
                    dimensions.y1,
                );
                placement = Some(Point::new(dimensions.x1, dimensions.y1));
                let points = dimensions.point_set();
                mb.monster_spawns.retain(|pt| !points.contains(pt));
            }

            attempts += 1;
        }

        if let Some(placement) = placement {
            let string_vec: Vec<char> = self
                .blueprint
                .chars()
                .filter(|a| *a != '\r' && *a != '\n')
                .collect();
            let mut i = 0;
            for ty in placement.y..placement.y + self.height {
                for tx in placement.x..placement.x + self.width {
                    let idx = map_idx(tx, ty);
                    let c = string_vec[i];
                    match c {
                        'M' => {
                            mb.map.tiles[idx] = TileType::Floor;
                            mb.monster_spawns.push(Point::new(tx, ty));
                        }
                        '-' => mb.map.tiles[idx] = TileType::Floor,
                        '#' => mb.map.tiles[idx] = TileType::Wall,
                        _ => println!("No idea what to do with [{c}]"),
                    }

                    i += 1;
                }
            }
        } else {
            println!("*** Unable to place prefab vault");
        }
    }
}
