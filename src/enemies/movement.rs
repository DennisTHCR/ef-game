use bevy::prelude::*;

use crate::structs::{
    markers::{EnemyMarker, PlayerMarker},
    mobs::Acceleration,
};

pub fn pathfind(
    mut enemies: Query<(&mut Acceleration, &Transform), With<EnemyMarker>>,
    player: Query<&Transform, With<PlayerMarker>>,
) {
    let player_translation = player.single().translation;
    enemies
        .iter_mut()
        .for_each(|(mut acceleration, transform)| {
            let delta_norm = (player_translation - transform.translation).normalize_or_zero();
            acceleration.0 += delta_norm.xy() * 1000.;
        });
}

pub fn forcefield(
    mut enemies: Query<(&mut Acceleration, &Transform), With<EnemyMarker>>,
    player: Query<&Transform, With<PlayerMarker>>
) {
    let player_translation = player.single().translation;
    enemies.iter_mut().for_each(|(mut acceleration, transform)| {
        let delta = player_translation - transform.translation;
        if delta.length() <= 20. {
            acceleration.0 += 20000. / -delta.xy();
        }
    });
}