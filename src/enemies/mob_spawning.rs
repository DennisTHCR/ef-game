use std::time::Duration;

use bevy::prelude::*;

use crate::structs::{
    markers::{EnemyMarker, SpawnerMarker},
    plugins::MobSpawnPlugin,
    world::SpawnerTimer,
    mobs::Health,
};

impl Plugin for MobSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_spawners);
    }
}

fn tick_spawners(
    mut spawners: Query<(&mut SpawnerTimer, &Transform), With<SpawnerMarker>>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    spawners.iter_mut().for_each(|(mut timer, &transform)| {
        timer.0.tick(Duration::from_secs_f32(time.delta_seconds()));
        if timer.0.just_finished() {
            let mut new_transform = transform;
            new_transform.translation.z = -0.5;
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("textures/enemy.png"),
                    transform: new_transform,
                    ..default()
                },
                EnemyMarker,
                Health::default_mob(),
            ));
        }
    });
}
