use crate::structs::{
    input::ParsedInput,
    markers::{PlayerMarker, ToolMarker},
    plugins::PlayerCombatPlugin,
};
use bevy::prelude::*;

impl Plugin for PlayerCombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_mouse);
    }
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
