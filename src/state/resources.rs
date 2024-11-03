use crate::prelude::*;

pub fn create_resources(mb: MapBuilder) -> Resources {
    let mut resources = Resources::default();
    resources.insert(mb.map);
    resources.insert(Camera::new(mb.player_start));
    resources.insert(TurnState::AwaitingInput);
    resources.insert(mb.theme);

    resources
}
