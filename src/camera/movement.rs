use bevy::prelude::*;

use crate::structs::{
    markers::{MainCamera, PlayerMarker},
    plugins::CameraMovementPlugin,
};

impl Plugin for CameraMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, follow_player);
    }
}

fn follow_player(
    player_transform: Query<&Transform, (With<PlayerMarker>, Without<MainCamera>)>,
    mut camera_transform: Query<&mut Transform, (With<MainCamera>, Without<PlayerMarker>)>,
    time: Res<Time>,
) {
    let delta_pos = camera_transform.single().translation - player_transform.single().translation;
    camera_transform.single_mut().translation -= delta_pos * time.delta_seconds();
    if delta_pos.length() <= time.delta_seconds() {
        camera_transform.single_mut().translation = player_transform.single().translation;
    }
}
