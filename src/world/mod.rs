mod generation;

use generation::get_material;
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
    let texture_handle = asset_server.load("sprite_sheet.png");
    let texture_atlas_layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 8, 8, None, None);
    let texture_atlas_layout_handle = texture_atlases.add(texture_atlas_layout);
    let mut texture_atlas = TextureAtlas::from(texture_atlas_layout_handle.clone());
    let mut emeralds = 0;
    let mut diamonds = 0;
    let mut iron = 0;
    let mut coal = 0;
    for y in -640..=640 {
        for x in -640..=640 {
            texture_atlas.index = match get_material(x, y) {
                Material::GRASS => 0,
                Material::STONE => 1,
                Material::COAL => 2,
                Material::IRON => 3,
                Material::DIAMOND => 4,
                Material::EMERALD => 5
            };
            if texture_atlas.index == 5 {
                emeralds += 1;
            } else if texture_atlas.index == 4 {
                diamonds += 1;
            } else if texture_atlas.index == 3 {
                iron += 1;
            } else if texture_atlas.index == 2 {
                coal += 1;
            }
            commands.spawn((
                SpriteBundle {
                    texture: texture_handle.clone(),
                    transform: Transform::from_xyz((x * 16) as f32, (y * 16) as f32, -10.0),
                    ..default()
                },
                texture_atlas.clone(),
            ));
        }
    }
    println!("emeralds: {emeralds}");
    println!("diamonds: {diamonds}");
    println!("iron: {iron}");
    println!("coal: {coal}");
}
