use bevy::{prelude::*, utils::HashMap};

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub enum Material {
    GRASS,
    STONE,
    COAL,
    IRON,
    DIAMOND,
    EMERALD,
}

#[derive(Resource)]
pub struct WorldTextures {
    pub texture_handle: Handle<Image>,
    pub texture_atlas: TextureAtlas,
}

#[derive(Resource, Default)]
pub struct WorldEntities {
    pub entity_map: HashMap<(i32, i32), Entity>,
}

#[derive(Resource, Default)]
pub struct WorldMaterials {
    pub material_map: HashMap<(i32, i32), Material>,
}
