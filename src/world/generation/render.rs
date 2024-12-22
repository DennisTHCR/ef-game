use bevy::prelude::*;

use crate::structs::{
    assets::TextureHandles,
    markers::SpawnerMarker,
    plugins::WorldRenderPlugin,
    window::WindowInfo,
    world::{Material, SpawnerTimer, WorldEntities, WorldMaterials, WorldTextureAtlas},
};

impl Plugin for WorldRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(Update, render_tiles);
    }
}

fn init(mut commands: Commands) {
    commands.insert_resource(WorldEntities::default());
}

pub fn render_tiles(
    mut commands: Commands,
    window_info: Res<WindowInfo>,
    mut world_entities: ResMut<WorldEntities>,
    world_texture_atlas: Res<WorldTextureAtlas>,
    world_materials: Res<WorldMaterials>,
    texture_handles: Res<TextureHandles>,
) {
    let material_map = &world_materials.material_map;
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
                let texture_handle = texture_handles.world_sprites.clone();
                let mut texture_atlas = world_texture_atlas.0.clone();
                let material = material_map.get(&(x, y));
                if material.is_some() {
                    let index = match material.unwrap() {
                        Material::Grass => 0,
                        Material::Stone => 1,
                        Material::Coal => 2,
                        Material::Iron => 3,
                        Material::Diamond => 4,
                        Material::Emerald => 5,
                        Material::Spawner => 6,
                    };
                    texture_atlas.index = index;
                    let mut entity = commands.spawn((
                        SpriteBundle {
                            texture: texture_handle,
                            // magic number 8.0 (sprite width/height = 16, transform is at center, this fixes the offset)
                            transform: Transform::from_xyz(
                                (x * 16) as f32 - 8.0,
                                (y * 16) as f32 - 8.0,
                                -1.,
                            ),
                            ..default()
                        },
                        texture_atlas,
                    ));
                    if index == 6 {
                        entity.insert((SpawnerMarker, SpawnerTimer::default()));
                    }
                    entity_map.insert((x, y), entity.id());
                }
            }
        }
    }
}
