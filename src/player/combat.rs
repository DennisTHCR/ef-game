use crate::structs::{
    input::ParsedInput,
    markers::{EnemyMarker, PlayerMarker},
    mobs::Health,
    player::PlayerStats,
    plugins::PlayerCombatPlugin,
};
use bevy::prelude::*;

impl Plugin for PlayerCombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_mouse, regenerate));
    }
}

fn regenerate(
    mut health: Query<&mut Health, With<PlayerMarker>>,
    player_stats: Res<PlayerStats>,
    time: Res<Time>,
) {
    health.single_mut().0 += health.single().0 * player_stats.regeneration * time.delta_seconds();
    health.single_mut().0 = health.single().0.clamp(0.0, player_stats.max_health);
}

fn handle_mouse(
    player_stats: Res<PlayerStats>,
    parsed_input: Res<ParsedInput>,
    player_transform: Query<&Transform, (With<PlayerMarker>, Without<EnemyMarker>)>,
    mut enemies: Query<(&mut Health, &Transform), (With<EnemyMarker>, Without<PlayerMarker>)>,
) {
    if !parsed_input.left_click || player_stats.selected_tool != 2 {
        return;
    }
    let player_translation = player_transform.single().translation;
    enemies.iter_mut().for_each(|(mut health, transform)| {
        if (player_translation - transform.translation).xy().length() <= player_stats.punch_range {
            health.0 -= player_stats.punch_force;
            println!("New health: {}", health.0);
        }
    });
}
