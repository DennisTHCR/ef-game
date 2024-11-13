use bevy::prelude::*;

use crate::structs::{
    plugins::WorldRenderPlugin,
    window::WindowInfo,
    world::{Material, WorldEntities, WorldTextures},
};

use super::util::get_material;

impl Plugin for WorldRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(Update, generate_tiles);
    }
}

fn init(mut commands: Commands) {
    commands.insert_resource(WorldEntities::default());
}

fn generate_tiles(
    mut commands: Commands,
    window_info: Res<WindowInfo>,
    mut world_entities: ResMut<WorldEntities>,
    world_textures: Res<WorldTextures>,
) {
    let entity_map = &mut world_entities.entity_map;
    let corners = &window_info.corner_coords;
    let start_x = (corners.min_x / 16.).floor() as i32;
    let end_x = (corners.max_x / 16.).ceil() as i32;
    let start_y = (corners.min_y / 16.).floor() as i32;
    let end_y = (corners.max_y / 16.).ceil() as i32;
    // MAGIC NUMBER 8 TO PREVENT PLAYER FROM SEEING WORLD GENERATION
    for y in end_y - 8..=start_y + 8 {
        for x in start_x - 8..=end_x + 8 {
            if entity_map.get(&(x, y)).is_none() {
                let texture_handle = world_textures.texture_handle.clone();
                let mut texture_atlas = world_textures.texture_atlas.clone();
                texture_atlas.index = match get_material(x, y) {
                    Material::GRASS => 0,
                    Material::STONE => 1,
                    Material::COAL => 2,
                    Material::IRON => 3,
                    Material::DIAMOND => 4,
                    Material::EMERALD => 5,
                };
                let entity = commands.spawn((
                    SpriteBundle {
                        texture: texture_handle,
                        transform: Transform::from_xyz((x * 16) as f32, (y * 16) as f32, -1.),
                        ..default()
                    },
                    texture_atlas,
                ));
                entity_map.insert((x, y), entity.id());
            }
        }
    }
}
