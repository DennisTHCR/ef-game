use bevy::{prelude::*, window::PrimaryWindow};

use crate::structs::{input::ParsedInput, markers::MainCamera, plugins::CursorPlugin};

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_cursor_resource);
    }
}

fn update_cursor_resource(
    mut parsed_input: ResMut<ParsedInput>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();
    if window.cursor_position().is_some() {
        parsed_input.cursor_position = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
            .unwrap_or_default();
    }
}
