use std::collections::HashMap;

use bevy::prelude::*;

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

#[derive(Component)]
pub struct WeaponMarker;
