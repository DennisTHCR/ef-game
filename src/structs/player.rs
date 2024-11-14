use bevy::{prelude::Resource, utils::HashMap};

use super::world::Material;

#[derive(Resource)]
pub struct PlayerSettings {
    pub speed: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        PlayerSettings { speed: 100. }
    }
}

#[derive(Resource, Default)]
struct ResourcesAvailable {
    pub resource_map: HashMap<Material, i128>,
}
