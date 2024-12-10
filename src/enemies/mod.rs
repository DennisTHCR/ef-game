mod mob_spawning;

use bevy::prelude::*;

use crate::structs::{
    markers::{EnemyMarker, PlayerMarker}, mobs::{AttackTimer, Health}, plugins::{EnemyPlugin, MobSpawnPlugin}
};

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_enemies, damage_player))
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

fn damage_player(
    mut enemies: Query<(&Transform, &mut AttackTimer), (With<EnemyMarker>, Without<PlayerMarker>)>,
    mut player: Query<(&mut Health, &Transform), (With<PlayerMarker>, Without<EnemyMarker>)>,
    time: Res<Time>,
) {
    let player_translation = player.single().1.translation;
    enemies.iter_mut().for_each(|(enemy_transform, mut attack_timer)| {
        attack_timer.0.tick(time.delta());
        let diff = enemy_transform.translation - player_translation;
        if diff.length() <= 20. {
            if attack_timer.0.just_finished() {
                player.single_mut().0.current_health -= 1;
                println!("Player Health: {}", player.single_mut().0.current_health);
                attack_timer.0.reset();
            }
        } else {
            attack_timer.0.reset();
        }
    });
}