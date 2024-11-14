use std::f32::consts::PI;

use bevy::prelude::*;

use crate::structs::{
    camera::CameraSettings,
    input::{CursorPos, KeyboardInput, MouseInput},
    markers::PlayerMarker,
    player::PlayerSettings,
    plugins::PlayerMovementPlugin,
};

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_keyboard, handle_mouse));
    }
}

fn handle_keyboard(
    mut transform: Query<&mut Transform, With<PlayerMarker>>,
    keyboard_input: Res<KeyboardInput>,
    player_settings: Res<PlayerSettings>,
    time: Res<Time>,
) {
    transform.single_mut().translation +=
        keyboard_input.direction.extend(0.0) * player_settings.speed * time.delta_seconds();
}

fn handle_mouse(
    mut transform: Query<&mut Transform, With<PlayerMarker>>,
    cursor_coords: Res<CursorPos>,
    mouse_input: Res<MouseInput>,
    mut camera_settings: ResMut<CameraSettings>,
) {
    let translation = transform.single().translation;
    let dx = cursor_coords.0.x - translation.x;
    let dy = cursor_coords.0.y - translation.y;
    transform.single_mut().rotation = Quat::from_rotation_z(dy.atan2(dx) - PI * 0.5);

    if mouse_input.scroll_down {
        camera_settings.scale /= 1.1;
    } else if mouse_input.scroll_up {
        camera_settings.scale *= 1.1;
    }
}
