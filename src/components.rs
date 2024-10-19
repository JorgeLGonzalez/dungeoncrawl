pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

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
pub struct WantsToMove {
    pub destination: Point,
    pub entity: Entity,
}

impl WantsToMove {
    pub fn new(destination: Point, entity: Entity) -> Self {
        Self {
            entity,
            destination,
        }
    }
}
