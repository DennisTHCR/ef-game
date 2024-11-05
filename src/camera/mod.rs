mod movement;

use crate::structs::{
    camera::CameraSettings,
    markers::MainCamera,
    plugins::{CameraMovementPlugin, CameraPlugin},
};
use bevy::prelude::*;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraMovementPlugin)
            .add_systems(Startup, init)
            .add_systems(Update, update_zoom)
            .insert_resource(Msaa::Off);
    }
}

fn init(mut commands: Commands) {
    let camera_settings = CameraSettings::default();
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                near: -1000.,
                far: 1000.,
                scale: camera_settings.scale,
                ..default()
            },
            ..default()
        },
        MainCamera,
    ));
    commands.insert_resource(camera_settings);
}

fn update_zoom(
    mut camera: Query<&mut OrthographicProjection, With<MainCamera>>,
    camera_settings: Res<CameraSettings>,
) {
    camera.single_mut().scale = camera_settings.scale;
}
