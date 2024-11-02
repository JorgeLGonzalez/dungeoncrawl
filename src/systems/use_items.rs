use super::helpers::ActivationMessage;
use crate::prelude::*;

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
        activation.activate(map, ecs);
        activation.remove(commands);
    }
}
