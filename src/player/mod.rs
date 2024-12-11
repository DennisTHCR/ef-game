mod movement;
mod resources;

use crate::{
    assets::init_textures,
    structs::{
        assets::TextureHandles,
        markers::PlayerMarker,
        mobs::Health,
        player::PlayerSettings,
        plugins::{PlayerMovementPlugin, PlayerPlugin, PlayerResourcePlugin},
    },
};
use bevy::prelude::*;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerMovementPlugin)
            .add_plugins(PlayerResourcePlugin)
            .add_systems(Startup, init.after(init_textures));
    }
}

fn init(mut commands: Commands, texture_handles: Res<TextureHandles>) {
    commands.insert_resource(PlayerSettings::default());
    commands.spawn((
        SpriteBundle {
            texture: texture_handles.player_sprite.clone(),
            ..default()
        },
        PlayerMarker,
        Health::default(),
    ));
}
