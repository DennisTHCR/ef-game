mod mob_spawning;

use bevy::prelude::*;

use crate::structs::{markers::{EnemyMarker, PlayerMarker}, plugins::{EnemyPlugin, MobSpawnPlugin}};

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_enemies)
            .add_plugins(MobSpawnPlugin);
    }
}

fn move_enemies(
    mut enemies: Query<&mut Transform, (With<EnemyMarker>, Without<PlayerMarker>)>,
    player_transform: Query<&Transform, (With<PlayerMarker>, Without<EnemyMarker>)>,
    time: Res<Time>,
) {
    let player_translation = player_transform.single().translation;
    enemies.iter_mut().for_each(|mut enemy_transform| {
        let diff = enemy_transform.translation - player_translation;
        let diff_normalized = diff.normalize_or_zero();
        enemy_transform.translation -= diff_normalized * time.delta_seconds() * 80.0;
    });
}
