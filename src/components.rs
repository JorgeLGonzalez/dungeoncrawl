pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AmuletOfYala; // YALA = Yet Another Lost Amulet :)

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(current: i32, max: i32) -> Self {
        Self { current, max }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

impl Render {
    pub fn new(color: ColorPair, glyph: FontCharType) -> Self {
        Self { color, glyph }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

impl WantsToAttack {
    pub fn new(attacker: Entity, victim: Entity) -> Self {
        Self { attacker, victim }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
