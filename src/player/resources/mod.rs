use bevy::prelude::*;

use crate::structs::{
    input::ParsedInput,
    player::AvailableResources,
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
) {
    if !parsed_input.left_click {
        return;
    }
    let material_map = &world_materials.material_map;
    let pos = parsed_input.cursor_position;
    let x = (pos.x / 16.) as i32;
    let y = (pos.y / 16.) as i32;
    let material = *material_map.get(&(x, y)).unwrap();
    if material == Material::Grass {
        return;
    }
    let resource_map = &mut available_resources.resource_map;
    if resource_map.get(&material).is_none() {
        resource_map.insert(material, 0);
    }
    resource_map.insert(material, resource_map[&material] + 1);
}
