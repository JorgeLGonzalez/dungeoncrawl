use crate::prelude::*;

pub struct Spawner<'a> {
    ecs: &'a mut World,
    rng: &'a mut RandomNumberGenerator,
}

impl<'a> Spawner<'a> {
    pub fn spawn(
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        map_builder: &mut MapBuilder,
        level: u32,
    ) {
        let mut spawner = Spawner { ecs, rng };

        if level == 2 {
            spawner.spawn_amulet_of_yala(map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        map_builder
            .monster_spawns
            .iter()
            .for_each(|pos| spawner.spawn_entity(*pos));

        if level == 0 {
            spawner.spawn_player(map_builder.player_start);
        }
    }

    fn spawn_entity(&mut self, pos: Point) {
        let roll = self.rng.roll_dice(1, 6);
        match roll {
            1 => self.spawn_healing_potion(pos),
            2 => self.spawn_magic_mapper(pos),
            _ => self.spawn_monster(pos),
        };
    }

    fn spawn_amulet_of_yala(&mut self, pos: Point) {
        self.ecs.push((
            Item,
            AmuletOfYala,
            pos,
            Render::new(
                ColorPair::new(WHITE, BLACK),
                to_cp437('/'),
                RenderOrder::Item,
            ),
        ));
    }

    fn spawn_healing_potion(&mut self, pos: Point) {
        self.ecs.push((
            Item,
            pos,
            Render::new(
                ColorPair::new(WHITE, BLACK),
                to_cp437('!'),
                RenderOrder::Item,
            ),
            Name("Healing Potion".to_string()),
            ProvidesHealing::new(6),
        ));
    }

    fn spawn_magic_mapper(&mut self, pos: Point) {
        self.ecs.push((
            Item,
            pos,
            Render::new(
                ColorPair::new(WHITE, BLACK),
                to_cp437('{'),
                RenderOrder::Item,
            ),
            Name("Dungeon Map".to_string()),
            ProvidesDungeonMap,
        ));
    }

    fn spawn_monster(&mut self, pos: Point) {
        let (hp, name, glyph) = match self.rng.roll_dice(1, 10) {
            1..=8 => goblin(),
            _ => orc(),
        };

        self.ecs.push((
            ChasingPlayer {},
            Enemy,
            FieldOfView::new(6),
            Health::new(hp, hp),
            Name(name),
            pos,
            Render::new(ColorPair::new(WHITE, BLACK), glyph, RenderOrder::Enemy),
        ));
    }

    fn spawn_player(&mut self, pos: Point) {
        self.ecs.push((
            FieldOfView::new(8),
            Health::new(10, 10),
            Player::default(),
            pos,
            Render::new(
                ColorPair::new(WHITE, BLACK),
                to_cp437('@'),
                RenderOrder::Player,
            ),
        ));
    }
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}
