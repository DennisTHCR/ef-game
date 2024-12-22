mod combat;
mod equipment;
mod movement;
mod resources;

use crate::{
    assets::init_textures,
    structs::{
        assets::TextureHandles,
        markers::{PlayerMarker, ToolMarker},
        mobs::Health,
        player::{PlayerEquipmentAtlas, PlayerStats},
        plugins::{PlayerCombatPlugin, PlayerMovementPlugin, PlayerPlugin, PlayerResourcePlugin},
        ui::HealthDisplay,
    },
};
use bevy::prelude::*;
use equipment::{change_tool, init_texture_atlas};

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerMovementPlugin)
            .add_plugins(PlayerResourcePlugin)
            .add_plugins(PlayerCombatPlugin)
            .add_systems(
                Startup,
                (init_texture_atlas, init).chain().after(init_textures),
            )
            .add_systems(Update, (update_health, change_tool));
    }
}

fn init(
    mut commands: Commands,
    texture_handles: Res<TextureHandles>,
    texture_atlas: Res<PlayerEquipmentAtlas>,
) {
    commands.insert_resource(PlayerStats::default());
    commands
        .spawn((
            SpriteBundle {
                texture: texture_handles.player_sprite.clone(),
                ..default()
            },
            PlayerMarker,
            Health::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: texture_handles.tools_sprite.clone(),
                    transform: Transform::from_scale(Vec3::splat(0.5))
                        .with_translation(Vec3::new(-14., 4., 0.))
                        .with_rotation(Quat::from_rotation_z(-0.6)),
                    ..default()
                },
                ToolMarker,
                texture_atlas.0.clone(),
            ));
        });
}

fn update_health(
    mut text: Query<&mut Text, With<HealthDisplay>>,
    health: Query<&Health, With<PlayerMarker>>,
) {
    text.single_mut().sections[1].value = health.single().current_health.to_string();
}
