use bevy::prelude::*;

use crate::structs::{input::ParsedInput, plugins::KeyboardPlugin};

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}

fn update(mut parsed_input: ResMut<ParsedInput>, keys: Res<ButtonInput<KeyCode>>) {
    let number_keycodes = [
        KeyCode::Digit0,
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
    ];
    let mut direction: Vec2 = Vec2::default();
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    parsed_input.direction = direction.normalize_or_zero();
    parsed_input.toggle_inventory = keys.just_pressed(KeyCode::KeyI);
    keys.get_pressed().for_each(|key_code| {
        if !number_keycodes.contains(key_code) {
            return;
        }
        parsed_input.selected_tool = match key_code {
            KeyCode::Digit0 => 0,
            KeyCode::Digit1 => 1,
            KeyCode::Digit2 => 2,
            KeyCode::Digit3 => 3,
            _ => -1,
        }
    });
}
