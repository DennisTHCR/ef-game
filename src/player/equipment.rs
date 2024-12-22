use bevy::prelude::*;

use crate::structs::{
    input::ParsedInput,
    markers::ToolMarker,
    player::{PlayerEquipmentAtlas, PlayerStats},
};

pub fn change_tool(
    input: Res<ParsedInput>,
    mut player_stats: ResMut<PlayerStats>,
    mut texture_atlas: Query<&mut TextureAtlas, With<ToolMarker>>,
) {
    if input.selected_tool == player_stats.selected_tool {
        return;
    }
    player_stats.selected_tool = input.selected_tool;
    texture_atlas.single_mut().index = input.selected_tool as usize;
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
