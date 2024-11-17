// use crate::prelude::*;
// use world::EntryRef;

// pub struct ActivationMessage {
//     item: Entity,
//     kind: ItemKind,
//     message: Entity,
//     user: Entity,
// }

// impl ActivationMessage {
//     pub fn new(activate: &ActivateItem, message: Entity, ecs: &SubWorld) -> Option<Self> {
//         if let Ok(item) = ecs.entry_ref(activate.item) {
//             determine_kind(item).map(|kind| ActivationMessage {
//                 item: activate.item,
//                 kind,
//                 message,
//                 user: activate.used_by,
//             })
//         } else {
//             None
//         }
//     }

//     pub fn activate(&self, map: &mut Map, ecs: &mut SubWorld) {
//         match self.kind {
//             ItemKind::Healing(h) => self.heal(h, ecs),
//             ItemKind::Map => self.reveal_map(map),
//         }
//     }

//     pub fn remove(&self, commands: &mut CommandBuffer) {
//         commands.remove(self.item);
//         commands.remove(self.message);
//     }

//     fn heal(&self, healing: ProvidesHealing, ecs: &mut SubWorld) {
//         if let Ok(mut target) = ecs.entry_mut(self.user) {
//             if let Ok(health) = target.get_component_mut::<Health>() {
//                 let heal_to = i32::min(health.max, health.current + healing.amount);
//                 println!(
//                     "Drank healing potion. Healed from {} to {}",
//                     health.current, heal_to
//                 );
//                 health.current = heal_to;
//             }
//         }
//     }

//     fn reveal_map(&self, map: &mut Map) {
//         map.revealed_tiles.iter_mut().for_each(|t| *t = true);
//         println!("Map has been revealed!");
//     }
// }

// fn determine_kind(item: EntryRef) -> Option<ItemKind> {
//     let healing = item
//         .get_component::<ProvidesHealing>()
//         .map(|&h| ItemKind::Healing(h))
//         .ok();

//     if healing.is_some() {
//         healing
//     } else {
//         item.get_component::<ProvidesDungeonMap>()
//             .map(|_| ItemKind::Map)
//             .ok()
//     }
// }

// enum ItemKind {
//     Healing(ProvidesHealing),
//     Map,
// }
