mod camera;
mod input;
mod player;
mod structs;

use bevy::prelude::*;
use structs::plugins::{CameraPlugin, InputPlugin, PlayerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(InputPlugin)
        .run();
}
