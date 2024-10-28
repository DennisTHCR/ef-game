pub mod cursor;

use bevy::prelude::*;
use cursor::CursorPlugin;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(CursorPlugin);
    }
}