mod mob_spawning;

use bevy::prelude::*;

use crate::structs::{
    markers::{EnemyMarker, PlayerMarker},
    mobs::{AttackTimer, DeathTimer, Health},
    plugins::{EnemyPlugin, MobSpawnPlugin},
    state::GameState,
};

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_enemies, damage_player, tick_death_timer).run_if(in_state(GameState::Playing)),
        )
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
        let mut movement = diff_normalized * time.delta_seconds() * -80.0;
        if diff.length() <= 100. {
            movement += diff_normalized * time.delta_seconds() * (2000. / diff.length()).clamp(0., 200.); 
        }
        enemy_transform.translation += movement;
    });
}

fn damage_player(
    mut enemies: Query<(&mut Transform, &mut AttackTimer, &mut Health, Entity), (With<EnemyMarker>, Without<PlayerMarker>)>,
    mut player: Query<(&mut Health, &Transform), (With<PlayerMarker>, Without<EnemyMarker>)>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let player_translation = player.single().1.translation;
    enemies
        .iter_mut()
        .for_each(|(mut enemy_transform, mut attack_timer, mut health, entity)| {
            attack_timer.0.tick(time.delta());
            let diff = enemy_transform.translation - player_translation;
            if diff.length() <= 30. {
                if attack_timer.0.just_finished() {
                    let diff = player_translation - enemy_transform.translation;
                    enemy_transform.translation += diff.normalize() * 20.;
                    player.single_mut().0.current_health -= 1;
                    health.current_health -= 1;
                    if health.current_health <= 0 {
                        commands.entity(entity).insert(DeathTimer::default());
                    }
                    if player.single().0.current_health <= 0 {
                        next_state.set(GameState::Paused);
                    }
                    attack_timer.0.reset();
                }
            } else {
                attack_timer.0.reset();
            }
        });
}

fn tick_death_timer(
    mut enemies: Query<(&mut DeathTimer, Entity, &mut Sprite), With<EnemyMarker>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    enemies.iter_mut().for_each(|(mut death_timer, entity, mut sprite)| {
        death_timer.0.tick(time.delta());
        if death_timer.0.just_finished() {
            commands.entity(entity).remove::<EnemyMarker>();
            sprite.color = Color::srgba(0.2, 0.2, 0.2, 0.2);
        }
    });
}