mod end_screens;
mod level_advancer;
mod spawner;

use crate::{
    prelude::*,
    systems::{build_input_scheduler, build_monster_scheduler, build_player_scheduler},
};
use level_advancer::advance_level;
use spawner::Spawner;
use std::{collections::HashSet, process::Command};

pub struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    monster_systems: Schedule,
    player_systems: Schedule,
}

impl State {
    pub fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mut mb = MapBuilder::new(&mut rng);
        let mut ecs = World::default();
        Spawner::spawn(&mut ecs, &mut rng, &mut mb, 0);
        let resources = create_resources(mb);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            monster_systems: build_monster_scheduler(),
            player_systems: build_player_scheduler(),
        }
    }

    fn advance_level(&mut self) {
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        let level = advance_level(&mut self.ecs, &map_builder);
        Spawner::spawn(&mut self.ecs, &mut rng, &mut map_builder, level);
        self.resources = create_resources(map_builder);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        end_screens::render_game_over(ctx);

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.restart();
        }
    }

    fn restart(&mut self) {
        Command::new("clear")
            .status()
            .expect("Failed to clear terminal");

        let mut rng = RandomNumberGenerator::new();
        let mut mb = MapBuilder::new(&mut rng);
        self.ecs = World::default();
        Spawner::spawn(&mut self.ecs, &mut rng, &mut mb, 0);
        self.resources = create_resources(mb);
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

        self.resources.insert(ctx.key);

        ctx.set_active_console(ConsoleLayer::Map.into());
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::NextLevel => self.advance_level(),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::Victory => self.victory(ctx),
        }

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn create_resources(mb: MapBuilder) -> Resources {
    let mut resources = Resources::default();
    resources.insert(mb.map);
    resources.insert(Camera::new(mb.player_start));
    resources.insert(TurnState::AwaitingInput);
    resources.insert(mb.theme);

    resources
}
