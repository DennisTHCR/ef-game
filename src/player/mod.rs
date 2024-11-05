mod movement;

use crate::structs::{
    markers::PlayerMarker,
    player::PlayerSettings,
    plugins::{PlayerMovementPlugin, PlayerPlugin},
};
use bevy::prelude::*;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerMovementPlugin)
            .add_systems(Startup, init);
    }
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PlayerSettings::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player.png"),
            ..default()
        },
        PlayerMarker,
    ));
}
