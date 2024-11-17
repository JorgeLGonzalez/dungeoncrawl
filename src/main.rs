mod camera;
mod components;
mod game_stage;
mod map;
mod map_builder;
mod state;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*; // force to import first

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::game_stage::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::turn_state::*;
    pub use bevy::prelude::*;
    pub use iyes_loopless::prelude::*;
    // pub use legion::world::SubWorld;
    // pub use legion::*;

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
use state::State;

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
