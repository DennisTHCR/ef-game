use bevy::prelude::*;
use util::get_material;

use crate::structs::{
    plugins::{WorldGenerationPlugin, WorldRenderPlugin},
    window::WindowInfo,
    world::WorldMaterials,
};

mod render;
pub mod util;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldRenderPlugin)
            .add_systems(Startup, init)
            .add_systems(Update, generate.before(render::render_tiles));
    }
}

fn init(mut commands: Commands) {
    commands.insert_resource(WorldMaterials::default());
}

fn generate(mut world_materials: ResMut<WorldMaterials>, window_info: Res<WindowInfo>) {
    let material_map = &mut world_materials.material_map;
    let corners = &window_info.corner_coords;
    let start_x = (corners.min_x / 16.).floor() as i32;
    let end_x = (corners.max_x / 16.).ceil() as i32;
    let start_y = (corners.min_y / 16.).floor() as i32;
    let end_y = (corners.max_y / 16.).ceil() as i32;
    // MAGIC NUMBER 8 TO PREVENT PLAYER FROM SEEING WORLD GENERATION
    for y in end_y - 8..=start_y + 8 {
        for x in start_x - 8..=end_x + 8 {
            if material_map.get(&(x, y)).is_none() {
                let material = get_material(x, y);
                material_map.insert((x, y), material);
            }
        }
    }
}
