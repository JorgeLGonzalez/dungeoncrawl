mod template;

use template::Templates;

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
        level: usize,
    ) {
        let mut spawner = Spawner { ecs, rng };

        if level == 2 {
            spawner.spawn_amulet_of_yala(map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        let template = Templates::load();
        template.spawn_entities(
            spawner.ecs,
            spawner.rng,
            level as usize,
            &map_builder.monster_spawns,
        );

        if level == 0 {
            spawner.spawn_player(map_builder.player_start);
        }
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

    fn spawn_player(&mut self, pos: Point) {
        self.ecs.push((
            Damage(1),
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
