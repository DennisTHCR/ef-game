use std::collections::HashMap;

use bevy::prelude::*;

use super::world::Material;

#[derive(Resource)]
pub struct PlayerStats {
    pub max_speed: f32,
    pub selected_tool: i32,
    pub mining_range: f32,
    pub punch_range: f32,
    pub mining_level: i32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        PlayerStats {
            max_speed: 100.,
            selected_tool: 0,
            mining_range: 20.,
            punch_range: 20.,
            mining_level: 0,
        }
    }
}

#[derive(Resource)]
pub struct AvailableResources {
    pub resource_map: HashMap<Material, i128>,
}

impl Default for AvailableResources {
    fn default() -> Self {
        AvailableResources {
            resource_map: HashMap::from_iter([
                (Material::Stone, 0),
                (Material::Coal, 0),
                (Material::Iron, 0),
                (Material::Diamond, 0),
                (Material::Emerald, 0),
            ]),
        }
    }
}

#[derive(Resource)]
pub struct PlayerEquipmentAtlas(pub TextureAtlas);
