use crate::structs::{
    input::ParsedInput, markers::{PlayerMarker, ToolMarker}, mobs::Health, player::PlayerStats, plugins::PlayerCombatPlugin
};
use bevy::prelude::*;

impl Plugin for PlayerCombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_mouse, regenerate));
    }
}

fn regenerate(mut health: Query<&mut Health, With<PlayerMarker>>, player_stats: Res<PlayerStats>, time: Res<Time>) {
    health.single_mut().0 += health.single().0 * player_stats.regeneration * time.delta_seconds();
    health.single_mut().0 = health.single().0.clamp(0.0, player_stats.max_health);
}

fn handle_mouse(
    parsed_input: Res<ParsedInput>,
    mut weapon: Query<&mut Transform, (With<ToolMarker>, Without<PlayerMarker>)>,
) {
    if parsed_input.left_click {
        weapon
            .single_mut()
            .rotate_around(Vec3::splat(0.), Quat::from_rotation_z(3.14));
    }
}
