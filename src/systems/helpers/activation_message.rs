use crate::prelude::*;
use world::EntryRef;

pub struct ActivationMessage {
    item: Entity,
    kind: ItemKind,
    message: Entity,
    user: Entity,
}

impl ActivationMessage {
    pub fn new(activate: &ActivateItem, message: Entity, ecs: &SubWorld) -> Option<Self> {
        if let Ok(item) = ecs.entry_ref(activate.item) {
            ActivationMessage::determine_kind(item).map(|kind| ActivationMessage {
                item: activate.item,
                kind,
                message,
                user: activate.used_by,
            })
        } else {
            None
        }
    }

    fn determine_kind(item: EntryRef) -> Option<ItemKind> {
        let healing = item
            .get_component::<ProvidesHealing>()
            .map(|&h| ItemKind::Healing(h))
            .ok();

        if healing.is_some() {
            healing
        } else {
            item.get_component::<ProvidesDungeonMap>()
                .map(|_| ItemKind::Map)
                .ok()
        }
    }

    pub fn activate(&self, map: &mut Map, ecs: &mut SubWorld) {
        match self.kind {
            ItemKind::Healing(h) => {
                if let Ok(mut target) = ecs.entry_mut(self.user) {
                    if let Ok(health) = target.get_component_mut::<Health>() {
                        health.current = i32::min(health.max, health.current + h.amount)
                    }
                }
            }
            ItemKind::Map => {
                map.revealed_tiles.iter_mut().for_each(|t| *t = true);
            }
        }
    }

    pub fn remove(&self, commands: &mut CommandBuffer) {
        commands.remove(self.item);
        commands.remove(self.message);
    }
}

enum ItemKind {
    Healing(ProvidesHealing),
    Map,
}
