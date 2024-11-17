use crate::prelude::*;
use legion::systems::CommandBuffer;
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
    pub fn spawn_entity(&self, pt: &Point, commands: &mut CommandBuffer) {
        // let entity = self.create_entity(pt, commands);
        // self.add_main_components(entity, commands);
        // self.add_effects(entity, commands);
        // self.add_damage(entity, commands);
    }

    fn add_damage(&self, entity: Entity, commands: &mut CommandBuffer) {
        if let Some(damage) = &self.base_damage {
            // commands.add_component(entity, Damage(*damage));
        }
    }

    fn add_effects(&self, entity: Entity, commands: &mut CommandBuffer) {
        if let Some(effects) = &self.provides {
            effects
                .iter()
                .for_each(|(provides, n)| match provides.as_str() {
                    // "Healing" => commands.add_component(entity, ProvidesHealing::new(*n)),
                    // "MagicMap" => commands.add_component(entity, ProvidesDungeonMap),
                    _ => println!("Warning: we don't know how to provide {provides}"),
                });
        }
    }

    fn add_main_components(&self, entity: Entity, commands: &mut CommandBuffer) {
        match self.entity_type {
            EntityType::Enemy => {
                // commands.add_component(entity, Enemy);
                // commands.add_component(entity, FieldOfView::new(6));
                // commands.add_component(entity, ChasingPlayer);
                // commands.add_component(entity, Health::new(self.hp.unwrap(), self.hp.unwrap()));
            }
            EntityType::Item => {
                // commands.add_component(entity, Item);
                // if self.base_damage.is_some() {
                //     commands.add_component(entity, Weapon);
                // }
            }
        }
    }

    // fn create_entity(&self, pt: &Point, commands: &mut CommandBuffer) -> Entity {
    //     commands.push((
    //         pt.clone(),
    //         Render::new(
    //             ColorPair::new(WHITE, BLACK),
    //             to_cp437(self.glyph),
    //             determine_render_order(&self.entity_type),
    //         ),
    //         Name(self.name.clone()),
    //     ))
    // }
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
