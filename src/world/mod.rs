use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

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
    let perlin = Perlin::new(1);
    let texture_handle = asset_server.load("sprite_sheet.png");
    let texture_atlas_layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 8, 8, None, None);
    let texture_atlas_layout_handle = texture_atlases.add(texture_atlas_layout);
    let mut texture_atlas = TextureAtlas::from(texture_atlas_layout_handle.clone());
    for y in -640..=640 {
        for x in -640..=640 {
            texture_atlas.index = (perlin.get([x as f64 / 16., y as f64 / 16.]) * 5.) as usize;
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
}
