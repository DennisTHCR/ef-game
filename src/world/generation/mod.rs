use bevy::prelude::*;

use crate::structs::plugins::{WorldGenerationPlugin, WorldRenderPlugin};

mod render;
mod util;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldRenderPlugin);
    }
}
