use bevy::prelude::*;

use crate::structs::plugins::WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
    }
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((SpriteBundle {
        texture: asset_server.load("player.png"),
        ..default()
    },));
}
