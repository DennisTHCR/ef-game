use std::path::Path;

use crate::structs::{
    assets::TextureHandles,
    plugins::AssetPlugin,
};
use bevy::prelude::*;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_textures);
    }
}

pub fn init_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_path = Path::new("textures");
    commands.insert_resource(TextureHandles {
        player_sprite: asset_server.load(texture_path.join("player.png")),
        enemy_sprite: asset_server.load(texture_path.join("enemy.png")),
        tools_sprite: asset_server.load(texture_path.join("tools.png")),
        world_sprites: asset_server.load(texture_path.join("sprite_sheet.png")),
    });
}
