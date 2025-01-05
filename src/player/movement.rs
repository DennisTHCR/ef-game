use std::f32::consts::PI;

use bevy::prelude::*;

use crate::structs::{
    input::ParsedInput,
    markers::PlayerMarker,
    mobs::{Acceleration, Velocity},
    plugins::PlayerMovementPlugin,
    state::GameState,
};

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_keyboard, handle_mouse).run_if(in_state(GameState::Playing)),
        );
    }
}

fn handle_keyboard(
    mut acceleration: Query<(&mut Acceleration, &mut Velocity), With<PlayerMarker>>,
    parsed_input: Res<ParsedInput>,
    time: Res<Time>,
) {
    let (mut acceleration, mut velocity) = acceleration.single_mut();
    acceleration.0 += parsed_input.direction * 150000. * time.delta_seconds();
    if parsed_input.direction.length() == 0. {
        if velocity.current.length() <= 0.5 {
            velocity.current = Vec2::ZERO;
        }
        acceleration.0 += -velocity.current * 15000. * time.delta_seconds();
    }
}

fn handle_mouse(
    mut transform: Query<&mut Transform, With<PlayerMarker>>,
    parsed_input: Res<ParsedInput>,
) {
    let translation = transform.single().translation;
    let dx = parsed_input.cursor_position.x - translation.x;
    let dy = parsed_input.cursor_position.y - translation.y;
    transform.single_mut().rotation = Quat::from_rotation_z(dy.atan2(dx) - PI * 0.5);
}
