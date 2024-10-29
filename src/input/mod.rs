mod cursor;
mod keyboard;

use crate::structs::plugins::{CursorPlugin, InputPlugin, KeyboardPlugin};
use bevy::prelude::*;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(KeyboardPlugin).add_plugins(CursorPlugin);
    }
}
