use bevy::{prelude::*, utils::HashMap};

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
