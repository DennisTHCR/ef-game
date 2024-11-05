use std::f32::consts::PI;

use bevy::prelude::*;

use crate::structs::{
    input::{CursorCoords, KeyboardInput},
    markers::PlayerMarker,
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
    time: Res<Time>,
) {
    transform.single_mut().translation +=
        keyboard_input.direction.extend(0.0) * 1000.0 * time.delta_seconds();
}

fn handle_mouse(
    mut transform: Query<&mut Transform, With<PlayerMarker>>,
    cursor_coords: Res<CursorCoords>,
) {
    let translation = transform.single().translation;
    let dx = cursor_coords.0.x - translation.x;
    let dy = cursor_coords.0.y - translation.y;
    transform.single_mut().rotation = Quat::from_rotation_z(dy.atan2(dx) - PI * 0.5);
}
