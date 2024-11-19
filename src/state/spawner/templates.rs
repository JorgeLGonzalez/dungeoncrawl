use super::template::Template;
use crate::prelude::*;
use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;

#[derive(Clone, Debug, Deserialize)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("resources/template.ron").expect("Failed opening file.");
        from_reader(file).expect("Unable to load templates")
    }

    pub fn spawn_entities(
        &self,
        world: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new();
        self.entities
            .iter()
            .filter(|e| e.levels.contains(&level))
            .for_each(|t| {
                for _ in 0..t.frequency {
                    available_entities.push(t);
                }
            });

        spawn_points.iter().for_each(|pt| {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                let mut world_spawner = world.spawn();
                entity.spawn_entity(pt, &mut world_spawner);
            }
        });
    }
}
