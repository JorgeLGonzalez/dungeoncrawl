use crate::components::Name as NameComponent;
use crate::prelude::*;

pub mod prelude {
    pub use super::HudInfo;
    pub use super::InventoryQuery;
    pub use super::PlayerQuery;
}

pub type InventoryQuery<'w, 's, 'n, 'c> =
    Query<'w, 's, (&'n NameComponent, &'c Carried), With<Item>>;
pub type PlayerQuery<'w, 's, 'p, 'h> = Query<'w, 's, (&'p Player, &'h Health), With<Player>>;

pub struct HudInfo {
    pub inventory: Vec<String>,
    pub map_level: usize,
    pub health: Health,
}

impl HudInfo {
    pub fn new(player_query: &PlayerQuery) -> Self {
        let (player, health) = player_query.single();

        Self {
            inventory: vec![],
            map_level: player.map_level,
            health: health.clone(),
        }
    }

    pub fn gather_inventory(self, inventory_query: &InventoryQuery) -> Self {
        let inventory = inventory_query
            .iter()
            .map(|(item, _)| item.0.clone())
            .collect();

        Self { inventory, ..self }
    }
}
