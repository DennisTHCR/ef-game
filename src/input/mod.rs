mod cursor;
mod keyboard;
mod mouse;

use crate::structs::{
    input::ParsedInput,
    plugins::{CursorPlugin, InputPlugin, KeyboardPlugin, MousePlugin},
};
use bevy::prelude::*;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_plugins(KeyboardPlugin)
            .add_plugins(CursorPlugin)
            .add_plugins(MousePlugin);
    }
}

fn init(mut commands: Commands) {
    commands.insert_resource(ParsedInput::default());
}
