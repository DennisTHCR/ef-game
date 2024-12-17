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
        ui::HealthDisplay,
    },
};
use bevy::prelude::*;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerMovementPlugin)
            .add_plugins(PlayerResourcePlugin)
            .add_systems(Startup, init.after(init_textures))
            .add_systems(Update, update_health);
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
    )).with_children(|parent| {
        parent.spawn(SpriteBundle {
            texture: texture_handles.sword_sprite.clone(),
            transform: Transform::from_scale(Vec3::splat(0.5)).with_translation(Vec3::new(10., 10.,0.)).with_rotation(Quat::from_rotation_z(-0.5)),
            ..default()
        });
    });
}

fn update_health(
    mut text: Query<&mut Text, With<HealthDisplay>>,
    health: Query<&Health, With<PlayerMarker>>,
) {
    text.single_mut().sections[1].value = health.single().current_health.to_string();
}
