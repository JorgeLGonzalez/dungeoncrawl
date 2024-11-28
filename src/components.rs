use crate::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Component, Copy, Debug, PartialEq)]
pub struct ActivateItem {
    pub item: Entity,
    pub used_by: Entity,
}

impl ActivateItem {
    pub fn new(item: Entity, used_by: Entity) -> Self {
        Self { item, used_by }
    }
}

#[derive(Component)]
pub struct AmuletOfYala; // YALA = Yet Another Lost Amulet :)

#[derive(Component)]
pub struct Carried(pub Entity);

#[derive(Component)]
pub struct ChasingPlayer;

#[derive(Component)]
pub struct Damage(pub i32);

#[derive(Component)]
pub struct Enemy;

#[derive(Clone, Component)]
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

#[derive(Clone, Copy, Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(current: i32, max: i32) -> Self {
        Self { current, max }
    }
}

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct MovingRandomly;

#[derive(Clone, Component)]
pub struct Name(pub String);

#[derive(Component, Default)]
pub struct Player {
    pub map_level: usize,
}

#[derive(Component)]
pub struct PointC(pub Point);

#[derive(Clone, Component, Copy)]
pub struct ProvidesHealing {
    pub amount: i32,
}

impl ProvidesHealing {
    pub fn new(amount: i32) -> Self {
        Self { amount }
    }
}

#[derive(Component)]
pub struct ProvidesDungeonMap;

#[derive(Clone, Component, Copy)]
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

#[derive(Component)]
pub struct Weapon;
