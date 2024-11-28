mod template;
mod templates;

use crate::prelude::*;
use templates::Templates;

pub struct Spawner<'a> {
    rng: &'a mut RandomNumberGenerator,
    world: &'a mut World,
}

impl<'a> Spawner<'a> {
    pub fn spawn(
        world: &mut World,
        rng: &mut RandomNumberGenerator,
        map_builder: &mut MapBuilder,
        level: usize,
    ) {
        let mut spawner = Spawner { rng, world };

        if level == 2 {
            spawner.spawn_amulet_of_yala(map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        let template = Templates::load();
        template.spawn_entities(
            spawner.world,
            spawner.rng,
            level as usize,
            &map_builder.monster_spawns,
        );

        if level == 0 {
            spawner.spawn_player(map_builder.player_start);
        }
    }

    fn spawn_amulet_of_yala(&mut self, pos: Point) {
        self.world.spawn().insert_bundle((
            Item,
            AmuletOfYala,
            PointC(pos),
            Render::new(
                ColorPair::new(WHITE, BLACK),
                to_cp437('|'),
                RenderOrder::Item,
            ),
        ));
    }

    fn spawn_player(&mut self, pos: Point) {
        self.world.spawn().insert_bundle((
            Damage(1),
            FieldOfView::new(8),
            Health::new(10, 10),
            Player::default(),
            PointC(pos),
            Render::new(
                ColorPair::new(WHITE, BLACK),
                to_cp437('@'),
                RenderOrder::Player,
            ),
        ));
    }
}

/*
In Bevy Entities are spawned as bundles. And it can be done declaratively by
deriving from Bundle.
See https://saveriomiroddi.github.io/learn_bevy_ecs_by_ripping_off/06_01/components_entities_resources.html#entities
*/
