mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::turn_state::*;
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const SCREEN_WIDTH: i32 = 80;

    pub enum ConsoleLayer {
        Map = 1,
        Entity = 2,
        Hud = 3,
    }

    impl From<ConsoleLayer> for usize {
        fn from(value: ConsoleLayer) -> Self {
            value as usize
        }
    }
}

use prelude::*;
use systems::{build_input_scheduler, build_monster_scheduler, build_player_scheduler};

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;

    main_loop(context, State::new())
}

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    monster_systems: Schedule,
    player_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let MapBuilder {
            amulet_start,
            map,
            player_start,
            rooms,
        } = MapBuilder::new(&mut rng);
        let resources = create_resources(map, player_start);
        let mut ecs = World::default();
        spawn(&mut ecs, &mut rng, player_start, amulet_start, &rooms);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            monster_systems: build_monster_scheduler(),
            player_systems: build_player_scheduler(),
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        render_game_over_overlay(ctx);

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.restart();
        }
    }

    fn restart(&mut self) {
        let mut rng = RandomNumberGenerator::new();
        let MapBuilder {
            amulet_start,
            map,
            player_start,
            rooms,
        } = MapBuilder::new(&mut rng);
        self.resources = create_resources(map, player_start);
        self.ecs = World::default();
        spawn(&mut self.ecs, &mut rng, player_start, amulet_start, &rooms);
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
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
        }

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn create_resources(map: Map, player_start: Point) -> Resources {
    let mut resources = Resources::default();
    resources.insert(map);
    resources.insert(Camera::new(player_start));
    resources.insert(TurnState::AwaitingInput);

    resources
}

fn render_game_over_overlay(ctx: &mut BTerm) {
    ctx.set_active_console(ConsoleLayer::Hud.into());
    ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
    ctx.print_color_centered(
        4,
        WHITE,
        BLACK,
        "Slain by a monster, your hero's journey has come to a premature end.",
    );
    ctx.print_color_centered(
        5,
        WHITE,
        BLACK,
        "The Amulet of Yala remains unclaimed, and your home town is not saved.",
    );
    ctx.print_color_centered(
        8,
        YELLOW,
        BLACK,
        "Don't worry, you can always try again with a new hero.",
    );
    ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again");
}
