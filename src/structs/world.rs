use bevy::{prelude::*, utils::HashMap};
use strum_macros::{Display, EnumIter};

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, EnumIter, Display, Component)]
pub enum Material {
    Grass,
    Spawner,
    Stone,
    Coal,
    Iron,
    Diamond,
    Emerald,
}

#[derive(Resource)]
pub struct WorldTextureAtlas(pub TextureAtlas);

#[derive(Resource, Default)]
pub struct WorldEntities {
    pub entity_map: HashMap<(i32, i32), Entity>,
}

#[derive(Resource, Default)]
pub struct WorldMaterials {
    pub material_map: HashMap<(i32, i32), Material>,
}

#[derive(Component)]
pub struct SpawnerTimer(pub Timer);

impl Default for SpawnerTimer {
    fn default() -> Self {
        SpawnerTimer(Timer::from_seconds(0.01, TimerMode::Repeating))
    }
}
