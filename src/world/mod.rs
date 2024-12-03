mod generation;

use crate::structs::{plugins::WorldGenerationPlugin, world::WorldTextures};
use bevy::prelude::*;

use crate::structs::plugins::WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_plugins(WorldGenerationPlugin);
    }
}

fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("textures/sprite_sheet.png");
    let texture_atlas_layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 8, 8, None, None);
    let texture_atlas_layout_handle = texture_atlases.add(texture_atlas_layout);
    let texture_atlas = TextureAtlas::from(texture_atlas_layout_handle.clone());
    let world_textures = WorldTextures {
        texture_handle,
        texture_atlas,
    };
    commands.insert_resource(world_textures);
}
