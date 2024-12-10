mod camera;
mod enemies;
mod input;
mod player;
mod structs;
mod ui;
mod window;
mod world;
mod assets;

use bevy::prelude::*;
use structs::plugins::{
    CameraPlugin, EnemyPlugin, InputPlugin, PlayerPlugin, UiPlugin, WindowInfoPlugin, WorldPlugin, AssetPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            AssetPlugin,
            PlayerPlugin,
            CameraPlugin,
            InputPlugin,
            WorldPlugin,
            WindowInfoPlugin,
            EnemyPlugin,
            UiPlugin,
        ))
        .run();
}
