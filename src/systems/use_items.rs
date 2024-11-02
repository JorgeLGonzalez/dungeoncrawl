use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[read_component(ProvidesDungeonMap)]
#[write_component(Health)]
pub fn use_items(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    let activations: Vec<ActivationMessage> = <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .filter_map(|(&entity, activate)| {
            if let Ok(item) = ecs.entry_ref(activate.item) {
                let kind = if let Ok(&healing) = item.get_component::<ProvidesHealing>() {
                    ItemKind::Healing(healing)
                } else if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>() {
                    ItemKind::Map
                } else {
                    ItemKind::None
                };

                Some(ActivationMessage {
                    item: activate.item,
                    kind,
                    message: entity,
                    user: activate.used_by,
                })
            } else {
                None
            }
        })
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
            ItemKind::None => (),
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

enum ItemKind {
    Healing(ProvidesHealing),
    Map,
    None,
}
