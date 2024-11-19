use crate::prelude::*;
use bevy::ecs::world::EntityMut;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Clone, Debug, Deserialize)]
pub struct Template {
    pub base_damage: Option<i32>,
    pub entity_type: EntityType,
    pub frequency: i32,
    pub glyph: char,
    pub hp: Option<i32>,
    pub levels: HashSet<usize>,
    pub name: String,
    pub provides: Option<Vec<(String, i32)>>,
}

impl Template {
    pub fn spawn_entity(&self, pt: &Point, world_spawner: &mut EntityMut) {
        let entity = world_spawner.insert_bundle((
            PointC(pt.clone()),
            Render::new(
                ColorPair::new(WHITE, BLACK),
                to_cp437(self.glyph),
                determine_render_order(&self.entity_type),
            ),
            Name(self.name.clone()),
        ));

        self.add_main_components(entity);
        self.add_effects(entity);
        self.add_damage(entity);
    }

    fn add_damage(&self, entity: &mut EntityMut) {
        if let Some(damage) = &self.base_damage {
            entity.insert(Damage(*damage));
        }
    }

    fn add_effects(&self, entity: &mut EntityMut) {
        if let Some(effects) = &self.provides {
            effects
                .iter()
                .for_each(|(provides, n)| match provides.as_str() {
                    "Healing" => {
                        entity.insert(ProvidesHealing::new(*n));
                    }
                    "MagicMap" => {
                        entity.insert(ProvidesDungeonMap);
                    }
                    _ => println!("Warning: we don't know how to provide {provides}"),
                });
        }
    }

    fn add_main_components(&self, entity: &mut EntityMut) {
        match self.entity_type {
            EntityType::Enemy => {
                entity
                    .insert(Enemy)
                    .insert(FieldOfView::new(6))
                    .insert(ChasingPlayer)
                    .insert(Health::new(self.hp.unwrap(), self.hp.unwrap()));
            }
            EntityType::Item => {
                entity.insert(Item);
                if self.base_damage.is_some() {
                    entity.insert(Weapon);
                }
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

fn determine_render_order(entity_type: &EntityType) -> RenderOrder {
    match entity_type {
        EntityType::Enemy => RenderOrder::Enemy,
        EntityType::Item => RenderOrder::Item,
    }
}
