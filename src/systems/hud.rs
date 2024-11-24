use super::helpers::hud_info::prelude::*;
use crate::prelude::*;

pub fn hud(inventory_query: InventoryQuery, player_query: PlayerQuery) {
    let info = HudInfo::new(&player_query).gather_inventory(&inventory_query);

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(ConsoleLayer::Hud.into());

    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        info.health.current,
        info.health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!("Health: {}/{}", info.health.current, info.health.max),
        ColorPair::new(WHITE, RED),
    );

    if !info.inventory.is_empty() {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }
    info.inventory.iter().enumerate().for_each(|(idx, item)| {
        draw_batch.print(Point::new(3, 3 + idx), format!("{}:{}", idx + 1, item));
    });

    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 1),
        format!("Dungeon Level: {}", info.map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    draw_batch.submit(10000).expect("Batch error");
}
