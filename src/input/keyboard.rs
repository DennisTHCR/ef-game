use bevy::prelude::*;

use crate::structs::{input::KeyboardInput, plugins::KeyboardPlugin};

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init).add_systems(Update, update);
    }
}

fn init(mut commands: Commands) {
    commands.insert_resource(KeyboardInput::default());
}

fn update(mut keyboard_input: ResMut<KeyboardInput>, keys: Res<ButtonInput<KeyCode>>) {
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
    keyboard_input.direction = direction.normalize_or_zero();
    keyboard_input.toggle_inventory = keys.just_pressed(KeyCode::KeyI);
}
