mod movement;

use crate::structs::{
    input::CursorCoords,
    markers::PlayerMarker,
    plugins::{PlayerMovementPlugin, PlayerPlugin},
};
use bevy::prelude::*;
use std::f32::consts::PI;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerMovementPlugin)
            .add_systems(Startup, init)
            .add_systems(Update, update);
    }
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player.png"),
            ..default()
        },
        PlayerMarker,
    ));
}

fn update(
    mut transform: Query<&mut Transform, With<PlayerMarker>>,
    cursor_coords: Res<CursorCoords>,
) {
    let translation = transform.single().translation;
    let dx = cursor_coords.0.x - translation.x;
    let dy = cursor_coords.0.y - translation.y;
    transform.single_mut().rotation = Quat::from_rotation_z(dy.atan2(dx) - PI * 0.5);
}
