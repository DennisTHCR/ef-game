use bevy::{prelude::Resource, utils::HashMap};

use super::world::Material;

#[derive(Resource)]
pub struct PlayerSettings {
    pub speed: f32,
    pub base_health: f32,
    pub base_regeneration: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        PlayerSettings {
            speed: 100.,
            base_health: 100.,
            base_regeneration: 0.,
        }
    }
}

#[derive(Resource, Default)]
pub struct AvailableResources {
    pub resource_map: HashMap<Material, i128>,
}
