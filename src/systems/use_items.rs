use crate::prelude::*;
use world::EntryRef;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[read_component(ProvidesDungeonMap)]
#[write_component(Health)]
pub fn use_items(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    let activations: Vec<ActivationMessage> = <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .filter_map(|(&entity, activate)| ActivationMessage::new(activate, entity, ecs))
        .collect();

    for activation in activations.iter() {
        match activation.kind {
            ItemKind::Healing(h) => {
                if let Ok(mut target) = ecs.entry_mut(activation.user) {
                    if let Ok(health) = target.get_component_mut::<Health>() {
                        health.current = i32::min(health.max, health.current + h.amount)
                    }
                }
            }
            ItemKind::Map => {
                map.revealed_tiles.iter_mut().for_each(|t| *t = true);
            }
        }

        commands.remove(activation.item);
        commands.remove(activation.message);
    }
}

struct ActivationMessage {
    item: Entity,
    kind: ItemKind,
    message: Entity,
    user: Entity,
}

impl ActivationMessage {
    fn new(activate: &ActivateItem, message: Entity, ecs: &SubWorld) -> Option<Self> {
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
}

enum ItemKind {
    Healing(ProvidesHealing),
    Map,
}
