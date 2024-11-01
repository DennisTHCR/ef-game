mod movement;

use crate::structs::{
    markers::PlayerMarker,
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
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player.png"),
            ..default()
        },
        PlayerMarker,
    ));
}
