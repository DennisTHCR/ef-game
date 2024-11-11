mod generation;

use crate::structs::world::Material;
use bevy::prelude::*;

use crate::structs::plugins::WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
    }
}

fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("sprite_sheet.png");
    let texture_atlas_layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 8, 8, None, None);
    let texture_atlas_layout_handle = texture_atlases.add(texture_atlas_layout);
    // texture_atlas.id decides texture
    // Material::GRASS => 0,
    // Material::STONE => 1,
    // Material::COAL => 2,
    // Material::IRON => 3,
    // Material::DIAMOND => 4,
    // Material::EMERALD => 5,
    let mut texture_atlas = TextureAtlas::from(texture_atlas_layout_handle.clone());
}
