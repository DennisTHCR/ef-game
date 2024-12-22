use bevy::prelude::*;

use crate::structs::{
    markers::{EnemyMarker, PlayerMarker},
    mobs::{AttackTimer, DeathTimer, Health},
    state::GameState,
};

pub fn damage_player(
    mut enemies: Query<
        (&mut Transform, &mut AttackTimer, &mut Health, Entity),
        (With<EnemyMarker>, Without<PlayerMarker>),
    >,
    mut player: Query<(&mut Health, &Transform), (With<PlayerMarker>, Without<EnemyMarker>)>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let player_translation = player.single().1.translation;
    enemies.iter_mut().for_each(
        |(mut enemy_transform, mut attack_timer, mut health, entity)| {
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
        },
    );
}
