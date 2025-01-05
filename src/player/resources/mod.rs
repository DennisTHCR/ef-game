use bevy::prelude::*;

use crate::structs::{
    input::ParsedInput,
    markers::PlayerMarker,
    player::{AvailableResources, PlayerStats},
    plugins::PlayerResourcePlugin,
    state::GameState,
    world::{Material, WorldMaterials},
};

impl Plugin for PlayerResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(Update, gather.run_if(in_state(GameState::Playing)));
    }
}

fn init(mut commands: Commands) {
    commands.insert_resource(AvailableResources::default());
}

fn gather(
    mut available_resources: ResMut<AvailableResources>,
    parsed_input: Res<ParsedInput>,
    world_materials: Res<WorldMaterials>,
    mut player_stats: ResMut<PlayerStats>,
    player_transform: Query<&Transform, With<PlayerMarker>>,
) {
    let pos = parsed_input.cursor_position;
    let player_pos = player_transform.single().translation.xy();
    if !parsed_input.left_click
        || player_stats.selected_tool != 1
        || (pos - player_pos).length() > player_stats.range
    {
        return;
    }
    let material_map = &world_materials.material_map;
    let x = (pos.x / 16.) as i32;
    let y = (pos.y / 16.) as i32;
    let material = *material_map.get(&(x, y)).unwrap();
    if material == Material::Grass || material == Material::Spawner {
        return;
    }
    let level = match material {
        Material::Grass => -1,
        Material::Spawner => -1,
        Material::Stone => 0,
        Material::Coal => 1,
        Material::Iron => 2,
        Material::Diamond => 3,
        Material::Emerald => 4,
    };
    if level > player_stats.mining_level {
        return;
    }
    if level == player_stats.mining_level {
        player_stats.mining_level += 1;
        player_stats.range += 5.;
    }
    let resource_map = &mut available_resources.resource_map;
    if resource_map.get(&material).is_none() {
        resource_map.insert(material, 0);
    }
    resource_map.insert(material, resource_map[&material] + 1);
    player_stats.range += 0.1 * level as f32;
}
