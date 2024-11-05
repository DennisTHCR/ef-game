mod cursor;
mod keyboard;
mod mouse;

use crate::structs::plugins::{CursorPlugin, InputPlugin, KeyboardPlugin, MousePlugin};
use bevy::prelude::*;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(KeyboardPlugin)
            .add_plugins(CursorPlugin)
            .add_plugins(MousePlugin);
    }
}
