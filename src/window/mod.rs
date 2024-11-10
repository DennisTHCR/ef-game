use bevy::{prelude::*, window::PrimaryWindow};

use crate::structs::{markers::MainCamera, plugins::WindowInfoPlugin, window::WindowInfo};

impl Plugin for WindowInfoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init)
            .add_systems(Update, update);
    }
}

fn init(mut commands: Commands) {
    commands.insert_resource(WindowInfo::default());
}

fn update(mut window_info: ResMut<WindowInfo>, q_window: Query<&Window, With<PrimaryWindow>>, q_camera: Query<(&Camera ,&GlobalTransform), With<MainCamera>>) {
    let window = q_window.single();
    let (camera, camera_transform) = q_camera.single();
    let size = window.size();
    let corners = &mut window_info.corners;
    corners.top_left = camera.viewport_to_world_2d(camera_transform, Vec2::default()).unwrap_or_default();
    corners.bottom_right = camera.viewport_to_world_2d(camera_transform, size).unwrap_or_default();
    corners.top_right = camera.viewport_to_world_2d(camera_transform, Vec2::new(size.x, 0.)).unwrap_or_default();
    corners.bottom_left = camera.viewport_to_world_2d(camera_transform, Vec2::new(0., size.y)).unwrap_or_default();
}
