use bevy::prelude::*;

use crate::structs::{
    markers::{EnemyMarker, PlayerMarker},
    mobs::{AttackTimer, Health},
    state::GameState,
};

pub fn damage_player(
    mut enemies: Query<
        (&mut Transform, &mut AttackTimer, &mut Health),
        (With<EnemyMarker>, Without<PlayerMarker>),
    >,
    mut player: Query<(&mut Health, &Transform), (With<PlayerMarker>, Without<EnemyMarker>)>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    let player_translation = player.single().1.translation;
    enemies
        .iter_mut()
        .for_each(|(mut enemy_transform, mut attack_timer, mut health)| {
            attack_timer.0.tick(time.delta());
            let diff = enemy_transform.translation - player_translation;
            if diff.length() <= 30. {
                if attack_timer.0.just_finished() {
                    let diff = player_translation - enemy_transform.translation;
                    enemy_transform.translation += diff.normalize() * 20.;
                    player.single_mut().0 .0 -= 1.;
                    health.0 -= 1.;
                    if player.single().0 .0 <= 0. {
                        next_state.set(GameState::Paused);
                    }
                    attack_timer.0.reset();
                }
            } else {
                attack_timer.0.reset();
            }
        });
}
