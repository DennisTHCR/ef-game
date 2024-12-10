use std::path::Path;

use bevy::prelude::*;
use crate::structs::{assets::{MeshHandles, TextureHandles}, plugins::AssetPlugin};

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init_meshes, init_textures));
    }
}

pub fn init_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(MeshHandles {
        health_rectangle: meshes.add(Rectangle::new(100.0, 20.0))
    });
}

pub fn init_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_path = Path::new("textures");
    commands.insert_resource(TextureHandles {
        player_sprite: asset_server.load(texture_path.join("player.png")),
        enemy_sprite: asset_server.load(texture_path.join("enemy.png")),
    });
}