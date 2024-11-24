use crate::prelude::*;

pub mod prelude {
    pub use super::HealthQuery;
    pub use super::ItemActivator;
    pub use super::ItemKind;
    pub use super::ItemsQuery;
}

pub type HealthQuery<'w, 's, 'h> = Query<'w, 's, &'h mut Health>;
pub type ItemsQuery<'w, 's, 'h, 'm> =
    Query<'w, 's, (Option<&'h ProvidesHealing>, Option<&'m ProvidesDungeonMap>)>;

pub enum ItemKind {
    Healing(ProvidesHealing),
    Map,
}

pub struct ItemActivator {
    pub item: Entity,
    user: Entity,
}

impl ItemActivator {
    pub fn new(activate: &ActivateItem) -> Self {
        Self {
            item: activate.item,
            user: activate.used_by,
        }
    }

    pub fn determine_kind(&self, items_query: &ItemsQuery) -> Option<ItemKind> {
        items_query.get(self.item).ok().and_then(|(healing, map)| {
            healing.map_or_else(
                || map.map(|_| ItemKind::Map),
                |&h| Some(ItemKind::Healing(h)),
            )
        })
    }

    pub fn heal(&self, healing: &ProvidesHealing, health_query: &mut HealthQuery) {
        if let Ok(mut health) = health_query.get_mut(self.user) {
            let heal_to = i32::min(health.max, health.current + healing.amount);
            println!(
                "Drank healing potion. Healed from {} to {}",
                health.current, heal_to
            );
            health.current = heal_to;
        }
    }

    pub fn reveal_map(&self, map: &mut Map) {
        map.revealed_tiles.iter_mut().for_each(|t| *t = true);
        println!("Map has been revealed!");
    }
}
