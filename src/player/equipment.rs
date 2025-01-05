use bevy::prelude::*;

use crate::structs::{
    assets::MaterialHandles,
    input::ParsedInput,
    markers::{RangeDisplayMarker, ToolMarker},
    player::{PlayerEquipmentAtlas, PlayerStats},
};

pub fn change_tool(
    input: Res<ParsedInput>,
    mut player_stats: ResMut<PlayerStats>,
    mut query: Query<&mut TextureAtlas, With<ToolMarker>>,
) {
    let mut texture_atlas = query.single_mut();
    if input.selected_tool == player_stats.selected_tool {
        return;
    }
    player_stats.selected_tool = input.selected_tool;
    texture_atlas.index = input.selected_tool as usize;
}

pub fn change_range_indicator(
    player_stats: Res<PlayerStats>,
    material_handles: Res<MaterialHandles>,
    mut query: Query<(&mut Transform, &mut Handle<ColorMaterial>), With<RangeDisplayMarker>>,
) {
    let (mut transform, mut material) = query.single_mut();
    if player_stats.selected_tool == 1 {
        if *material != material_handles.blue {
            *material = material_handles.blue.clone();
        }
        if transform.scale.x != player_stats.mining_range {
            transform.scale = Vec3::splat(player_stats.mining_range);
        }
    }
    if player_stats.selected_tool == 2 {
        if *material != material_handles.red {
            *material = material_handles.red.clone();
        }
        if transform.scale.x != player_stats.punch_range {
            transform.scale = Vec3::splat(player_stats.punch_range);
        }
    }
}

pub fn init_texture_atlas(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_atlas_layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 4, 1, None, None);
    let texture_atlas_layout_handle = texture_atlases.add(texture_atlas_layout);
    let texture_atlas = TextureAtlas::from(texture_atlas_layout_handle);
    let player_equipment_atlas = PlayerEquipmentAtlas(texture_atlas);
    commands.insert_resource(player_equipment_atlas);
}
