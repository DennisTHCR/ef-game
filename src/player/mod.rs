mod movement;
mod resources;

use crate::structs::{
    markers::PlayerMarker,
    mobs::Health,
    player::PlayerSettings,
    plugins::{PlayerMovementPlugin, PlayerPlugin, PlayerResourcePlugin},
};
use bevy::prelude::*;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerMovementPlugin)
            .add_plugins(PlayerResourcePlugin)
            .add_systems(Startup, init);
    }
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PlayerSettings::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/player.png"),
            ..default()
        },
        PlayerMarker,
        Health::default(),
    ));
}
