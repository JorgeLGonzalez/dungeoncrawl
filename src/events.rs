use bevy::prelude::Entity;
use bracket_lib::prelude::Point;

#[derive(PartialEq)]
pub struct WantsToMove {
    pub destination: Point,
    pub entity: Entity,
}

impl WantsToMove {
    pub fn new(entity: Entity, destination: Point) -> Self {
        Self {
            entity,
            destination,
        }
    }
}

/*
See https://saveriomiroddi.github.io/learn_bevy_ecs_by_ripping_off/07_03/managed_event_passing.html#bevys-event-passing
 */
