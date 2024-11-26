mod camera;
mod enemies;
mod input;
mod player;
mod structs;
mod window;
mod world;

use bevy::prelude::*;
use structs::plugins::{
    CameraPlugin, EnemyPlugin, InputPlugin, PlayerPlugin, WindowInfoPlugin, WorldPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PlayerPlugin,
            CameraPlugin,
            InputPlugin,
            WorldPlugin,
            WindowInfoPlugin,
            EnemyPlugin,
        ))
        .run();
}
