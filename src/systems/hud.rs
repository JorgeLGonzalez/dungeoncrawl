use crate::components::Name as NameComponent;
use crate::prelude::*;

pub fn hud(
    player_query: Query<(&Player, &Health), With<Player>>,
    inventory_query: Query<(&NameComponent, &Carried), With<Item>>,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(ConsoleLayer::Hud.into());

    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");
    let (player, player_health) = player_query.single();
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!("Health: {}/{}", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );

    if !inventory_query.is_empty() {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }
    inventory_query
        .iter()
        .enumerate()
        .for_each(|(idx, (item, _))| {
            draw_batch.print(Point::new(3, 3 + idx), format!("{}:{}", idx + 1, item.0));
        });

    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 1),
        format!("Dungeon Level: {}", player.map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    draw_batch.submit(10000).expect("Batch error");
}

// struct HudInfo {
//     inventory: Vec<String>,
//     map_level: usize,
//     player_health: Health,
// }

// impl HudInfo {
//     fn new(ecs: &SubWorld) -> Self {
//         // let mut health_query = <&Health>::query().filter(component::<Player>());
//         // let player_health = *health_query.iter(ecs).nth(0).unwrap();

//         // let (player, map_level) = <(Entity, &Player)>::query()
//         //     .iter(ecs)
//         //     .find_map(|(entity, player)| Some((*entity, player.map_level)))
//         //     .unwrap();

//         // Self {
//         //     inventory: gather_inventory(player, ecs),
//         //     map_level,
//         //     player_health,
//         // }
//         Self {
//             inventory: Vec::new(),
//             map_level: 0,
//             player_health: Health::new(1, 1),
//         }
//     }
// }

// fn gather_inventory(player: Entity, ecs: &SubWorld) -> Vec<String> {
//     // <(&Item, &Name, &Carried)>::query()
//     //     .iter(ecs)
//     //     .filter(|(_, _, carried)| carried.0 == player)
//     //     .map(|(_, name, _)| name.0.clone())
//     //     .collect()
//     vec![]
// }
