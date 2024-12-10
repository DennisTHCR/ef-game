use bevy::prelude::*;

#[derive(Resource)]
pub struct MeshHandles {
    pub health_rectangle: Handle<Mesh>
}

#[derive(Resource)]
pub struct TextureHandles {
    pub player_sprite: Handle<Image>,
    pub enemy_sprite: Handle<Image>,
}