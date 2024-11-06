use std::collections::HashSet;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActivateItem {
    pub item: Entity,
    pub used_by: Entity,
}

impl ActivateItem {
    pub fn new(item: Entity, used_by: Entity) -> Self {
        Self { item, used_by }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AmuletOfYala; // YALA = Yet Another Lost Amulet :)

#[derive(Clone, Debug, PartialEq)]
pub struct Carried(pub Entity);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub is_dirty: bool,
    pub radius: i32,
    pub visible_tiles: HashSet<Point>,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            is_dirty: true,
            radius,
            visible_tiles: HashSet::new(),
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            is_dirty: true,
            radius: self.radius,
            visible_tiles: HashSet::new(),
        }
    }
}

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

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Player {
    pub map_level: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesHealing {
    pub amount: i32,
}

impl ProvidesHealing {
    pub fn new(amount: i32) -> Self {
        Self { amount }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesDungeonMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
    pub order: RenderOrder,
}

impl Render {
    pub fn new(color: ColorPair, glyph: FontCharType, order: RenderOrder) -> Self {
        Self {
            color,
            glyph,
            order,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RenderOrder {
    Enemy = 1,
    Item = 0,
    Player = 2,
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
