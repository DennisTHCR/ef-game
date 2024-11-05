mod movement;

use crate::structs::{
    markers::MainCamera,
    plugins::{CameraMovementPlugin, CameraPlugin},
};
use bevy::prelude::*;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraMovementPlugin)
            .add_systems(Startup, init)
            .insert_resource(Msaa::Off);
    }
}

fn init(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                near: -1000.,
                far: 1000.,
                scale: 1.,
                ..default()
            },
            ..default()
        },
        MainCamera,
    ));
}
