mod camera;
mod input;
mod player;
mod structs;
mod world;

use bevy::prelude::*;
use structs::plugins::{CameraPlugin, InputPlugin, PlayerPlugin, WorldPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PlayerPlugin,
            CameraPlugin,
            InputPlugin,
            WorldPlugin,
        ))
        .run();
}
