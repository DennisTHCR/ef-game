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
    let size = window.size();
    let coords = &mut window_info.corner_coords;
    let top_left = camera.viewport_to_world_2d(camera_transform, Vec2::new(0., 0.)).unwrap_or_default();
    let bottom_right = camera.viewport_to_world_2d(camera_transform, Vec2::new(size.x, size.y)).unwrap_or_default();
    coords.min_x = top_left.x;
    coords.max_x = bottom_right.x;
    coords.min_y = top_left.y;
    coords.max_y = bottom_right.y;
}
