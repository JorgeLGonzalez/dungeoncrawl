mod end_screens;
mod level_advancer;
mod spawner;

use crate::{prelude::*, systems::build_system_sets};
use level_advancer::advance_level;
use spawner::Spawner;

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

        ecs.add_event::<ActivateItem>()
            .add_event::<WantsToAttack>()
            .add_event::<WantsToMove>();

        ecs.add_stage_after(
            CoreStage::Update,
            GameStage::PlayerCombat,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::PlayerCombat,
            GameStage::MovePlayer,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::MovePlayer,
            GameStage::PlayerFov,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::PlayerFov,
            GameStage::GenerateMonsterMoves,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::GenerateMonsterMoves,
            GameStage::MonsterCombat,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::MonsterCombat,
            GameStage::MoveMonsters,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::MoveMonsters,
            GameStage::MonsterFov,
            SystemStage::parallel(),
        );

        build_system_sets(&mut ecs);

        let mut state = Self { ecs };
        state.create_resources(mb);

        state
    }

    fn advance_level(&mut self) {
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        let level = advance_level(&mut self.ecs.world, &map_builder);
        Spawner::spawn(&mut self.ecs.world, &mut rng, &mut map_builder, level);

        self.create_resources(map_builder);
    }

    fn create_resources(&mut self, map_builder: MapBuilder) {
        self.ecs
            .insert_resource(map_builder.map)
            .insert_resource(Camera::new(map_builder.player_start))
            .insert_resource(TurnState::AwaitingInput);
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

        ctx.set_active_console(ConsoleLayer::Map.into());
        self.ecs.insert_resource(Point::from_tuple(ctx.mouse_pos()));

        match self.ecs.world.get_resource::<TurnState>() {
            Some(TurnState::GameOver) => self.game_over(ctx),
            Some(TurnState::NextLevel) => self.advance_level(),
            Some(TurnState::Victory) => self.victory(ctx),
            _ => (),
        }

        self.ecs.update();

        render_draw_buffer(ctx).expect("Render error");
    }
}
