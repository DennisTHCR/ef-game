mod combat;
mod equipment;
mod movement;
mod resources;

use crate::{
    assets::init_assets,
    structs::{
        assets::{MaterialHandles, MeshHandles, TextureHandles},
        markers::{PlayerMarker, RangeDisplayMarker, ToolMarker},
        mobs::{Acceleration, Health, Velocity},
        player::{PlayerEquipmentAtlas, PlayerStats},
        plugins::{PlayerCombatPlugin, PlayerMovementPlugin, PlayerPlugin, PlayerResourcePlugin},
        ui::HealthDisplay,
    },
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use equipment::{change_range_indicator, change_tool, init_texture_atlas};

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerMovementPlugin)
            .add_plugins(PlayerResourcePlugin)
            .add_plugins(PlayerCombatPlugin)
            .add_systems(
                Startup,
                (init_texture_atlas, init).chain().after(init_assets),
            )
            .add_systems(Update, (update_health, change_tool, change_range_indicator));
    }
}

fn init(
    mut commands: Commands,
    texture_handles: Res<TextureHandles>,
    material_handles: Res<MaterialHandles>,
    mesh_handles: Res<MeshHandles>,
    texture_atlas: Res<PlayerEquipmentAtlas>,
) {
    let player_stats = PlayerStats::default();
    commands
        .spawn((
            SpriteBundle {
                texture: texture_handles.player_sprite.clone(),
                ..default()
            },
            PlayerMarker,
            Health(player_stats.max_health),
            Acceleration::default(),
            Velocity {
                max: player_stats.max_speed,
                ..default()
            },
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
            parent.spawn((
                MaterialMesh2dBundle {
                    mesh: mesh_handles.circle.clone(),
                    material: material_handles.blue.clone(),
                    transform: Transform::from_scale(Vec3::splat(40.)),
                    ..default()
                },
                RangeDisplayMarker,
            ));
        });
    commands.insert_resource(player_stats);
}

fn update_health(
    mut text: Query<&mut Text, With<HealthDisplay>>,
    health: Query<&Health, With<PlayerMarker>>,
) {
    text.single_mut().sections[1].value = health.single().0.to_string();
}
