use bevy::prelude::Entity;
use bracket_lib::prelude::Point;

#[derive(PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

impl WantsToAttack {
    pub fn new(attacker: Entity, victim: Entity) -> Self {
        Self { attacker, victim }
    }
}

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
