use bevy::{prelude::*, sprite::Mesh2dHandle};

#[derive(Resource)]
pub struct TextureHandles {
    pub player_sprite: Handle<Image>,
    pub tools_sprite: Handle<Image>,
    pub enemy_sprite: Handle<Image>,
    pub world_sprites: Handle<Image>,
}

#[derive(Resource)]

pub struct MeshHandles {
    pub circle: Mesh2dHandle,
}

#[derive(Resource)]
pub struct MaterialHandles {
    pub red: Handle<ColorMaterial>,
    pub blue: Handle<ColorMaterial>,
}
