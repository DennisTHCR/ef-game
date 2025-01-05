use std::path::Path;

use crate::structs::{
    assets::{MaterialHandles, MeshHandles, TextureHandles},
    plugins::AssetPlugin,
};
use bevy::prelude::*;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_assets);
    }
}

pub fn init_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_path = Path::new("textures");
    commands.insert_resource(TextureHandles {
        player_sprite: asset_server.load(texture_path.join("player.png")),
        enemy_sprite: asset_server.load(texture_path.join("enemy.png")),
        tools_sprite: asset_server.load(texture_path.join("tools.png")),
        world_sprites: asset_server.load(texture_path.join("sprite_sheet.png")),
    });
    commands.insert_resource(MeshHandles {
        circle: meshes.add(Annulus::new(0.9, 1.0)).into(),
    });
    commands.insert_resource(MaterialHandles {
        red: materials.add(Color::linear_rgba(1.0, 0.0, 0.0, 0.2)),
        blue: materials.add(Color::linear_rgba(0.0, 0.0, 1.0, 0.2)),
    });
}
