use bevy::{prelude::*, window::PrimaryWindow};

use crate::structs::{markers::MainCamera, plugins::WindowInfoPlugin, window::WindowInfo};

impl Plugin for WindowInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init).add_systems(Update, update);
    }
}

fn init(mut commands: Commands) {
    commands.insert_resource(WindowInfo::default());
}

fn update(
    mut window_info: ResMut<WindowInfo>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = q_window.single();
    let (camera, camera_transform) = q_camera.single();
    let translation = camera_transform.translation;
    let size = window.size();
    let coords = &mut window_info.corner_coords;
    coords.min_x = translation.x - size.x / 2.0;
    coords.max_x = translation.x + size.x / 2.0;
    coords.min_y = translation.y - size.y / 2.0;
    coords.max_y = translation.y + size.y / 2.0;
}
