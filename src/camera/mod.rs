use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init);    
    }
}

#[derive(Component)]
pub struct MainCamera;

fn init(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                near: -1000.,
                far: 1000.,
                scale: 0.2,
                ..default()
            },
            ..default()
        },
        MainCamera,
    ));
}
