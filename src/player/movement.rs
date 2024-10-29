use bevy::prelude::*;

use crate::structs::{input::KeyboardInput, markers::PlayerMarker, plugins::PlayerMovementPlugin};

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}

fn update(
    mut transform: Query<&mut Transform, With<PlayerMarker>>,
    keyboard_input: Res<KeyboardInput>,
    time: Res<Time>,
) {
    transform.single_mut().translation +=
        keyboard_input.direction.extend(0.0) * 100.0 * time.delta_seconds();
}
