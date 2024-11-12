use bevy::prelude::*;

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
    texture_handle: Handle<Image>,
    
}