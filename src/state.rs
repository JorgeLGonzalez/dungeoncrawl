mod end_screens;
mod level_advancer;
mod spawner;

use crate::{prelude::*, systems::build_system_sets};
use level_advancer::advance_level;
use spawner::Spawner;
use std::process::Command;

pub struct State {
    ecs: App,
}

impl State {
    pub fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mut mb = MapBuilder::new(&mut rng);
        let mut ecs = App::new();
        // This is not a strict-ECS approach (a system would), but we mimic the source project design.
        Spawner::spawn(&mut ecs.world, &mut rng, &mut mb, 0);

        ecs.insert_resource(mb.map);
        ecs.insert_resource(Camera::new(mb.player_start));

        ecs.add_event::<WantsToMove>();

        ecs.add_stage_after(
            CoreStage::Update,
            GameStage::MovePlayer,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::MovePlayer,
            GameStage::Collisions,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::Collisions,
            GameStage::MoveMonsters,
            SystemStage::parallel(),
        );

        ecs.insert_resource(TurnState::AwaitingInput);

        build_system_sets(&mut ecs);
        Self { ecs }
    }

    fn advance_level(&mut self) {
        // let mut rng = RandomNumberGenerator::new();
        // let mut map_builder = MapBuilder::new(&mut rng);

        // let level = advance_level(&mut self.ecs, &map_builder);
        // Spawner::spawn(&mut self.ecs, &mut rng, &mut map_builder, level);

        // self.create_resources(mb);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        end_screens::render_game_over(ctx);

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.restart();
        }
    }

    fn restart(&mut self) {
        // Command::new("clear")
        //     .status()
        //     .expect("Failed to clear terminal");

        // let mut rng = RandomNumberGenerator::new();
        // let mut mb = MapBuilder::new(&mut rng);
        // self.ecs = World::default();
        // Spawner::spawn(&mut self.ecs, &mut rng, &mut mb, 0);
        // self.resources = create_resources(mb);
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        end_screens::render_victory(ctx);

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.restart();
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(ConsoleLayer::Map.into());
        ctx.cls();
        ctx.set_active_console(ConsoleLayer::Entity.into());
        ctx.cls();
        ctx.set_active_console(ConsoleLayer::Hud.into());
        ctx.cls();

        if let Some(key) = ctx.key {
            self.ecs.insert_resource(key);
        } else {
            // In order to keep consistency with the Legion version, we need to access Bevy's World
            // directly, since App doesn't support removing resources.
            self.ecs.world.remove_resource::<VirtualKeyCode>();
        }

        // ctx.set_active_console(ConsoleLayer::Map.into());
        // self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        // let current_state = self.resources.get::<TurnState>().unwrap().clone();
        // match current_state {
        //     TurnState::AwaitingInput => self
        //         .input_systems
        //         .execute(&mut self.ecs, &mut self.resources),
        //     TurnState::GameOver => self.game_over(ctx),
        //     TurnState::MonsterTurn => self
        //         .monster_systems
        //         .execute(&mut self.ecs, &mut self.resources),
        //     TurnState::NextLevel => self.advance_level(),
        //     TurnState::PlayerTurn => self
        //         .player_systems
        //         .execute(&mut self.ecs, &mut self.resources),
        //     TurnState::Victory => self.victory(ctx),
        // }

        self.ecs.update();

        render_draw_buffer(ctx).expect("Render error");
    }
}
