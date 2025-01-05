use bevy::prelude::*;

#[derive(Resource)]
pub struct TextureHandles {
    pub player_sprite: Handle<Image>,
    pub tools_sprite: Handle<Image>,
    pub enemy_sprite: Handle<Image>,
    pub world_sprites: Handle<Image>,
}