use super::helpers::item_activator::prelude::*;
use crate::prelude::*;

pub fn use_items(
    mut activation_events: EventReader<ActivateItem>,
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut health_query: HealthQuery,
    items_query: ItemsQuery,
) {
    activation_events
        .iter()
        .map(ItemActivator::new)
        .for_each(|activator| {
            match activator.determine_kind(&items_query) {
                Some(ItemKind::Healing(h)) => activator.heal(&h, &mut health_query),
                Some(ItemKind::Map) => activator.reveal_map(&mut map),
                None => (),
            }

            commands.entity(activator.item).despawn();
        });
}
