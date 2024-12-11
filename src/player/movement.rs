use std::f32::consts::PI;

use bevy::prelude::*;

use crate::structs::{
    camera::CameraSettings, input::ParsedInput, markers::PlayerMarker, player::PlayerSettings,
    plugins::PlayerMovementPlugin, state::GameState,
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
    mut transform: Query<&mut Transform, With<PlayerMarker>>,
    parsed_input: Res<ParsedInput>,
    player_settings: Res<PlayerSettings>,
    time: Res<Time>,
) {
    transform.single_mut().translation +=
        parsed_input.direction.extend(0.0) * player_settings.speed * time.delta_seconds();
}

fn handle_mouse(
    mut transform: Query<&mut Transform, With<PlayerMarker>>,
    mut camera_settings: ResMut<CameraSettings>,
    parsed_input: Res<ParsedInput>,
) {
    let translation = transform.single().translation;
    let dx = parsed_input.cursor_position.x - translation.x;
    let dy = parsed_input.cursor_position.y - translation.y;
    transform.single_mut().rotation = Quat::from_rotation_z(dy.atan2(dx) - PI * 0.5);

    if parsed_input.scroll_down {
        camera_settings.scale /= 1.1;
    } else if parsed_input.scroll_up {
        camera_settings.scale *= 1.1;
    }
}
