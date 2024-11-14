use bevy::prelude::*;

use crate::structs::{
    input::{CursorPos, MouseInput},
    player::AvailableResources,
    plugins::PlayerResourcePlugin,
    world::{Material, WorldMaterials},
};

impl Plugin for PlayerResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init).add_systems(Update, gather);
    }
}

fn init(mut commands: Commands) {
    commands.insert_resource(AvailableResources::default());
}

fn gather(
    mut available_resources: ResMut<AvailableResources>,
    mouse_input: Res<MouseInput>,
    cursor_position: Res<CursorPos>,
    world_materials: Res<WorldMaterials>,
) {
    if !mouse_input.left_click {
        return;
    }
    let material_map = &world_materials.material_map;
    let pos = cursor_position.0;
    println!("x: {} y: {}", pos.x, pos.y);
    let x = (pos.x / 16.) as i32;
    let y = (pos.y / 16.) as i32;
    let material = material_map.get(&(x, y)).unwrap().clone();
    if material == Material::GRASS {
        return;
    }
    let resource_map = &mut available_resources.resource_map;
    if resource_map.get(&material).is_none() {
        resource_map.insert(material, 0);
    }
    resource_map.insert(material, resource_map[&material] + 1);
    println!("WOAH");
}
