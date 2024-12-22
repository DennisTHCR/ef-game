mod assets;
mod camera;
mod enemies;
mod input;
mod movement;
mod player;
mod state;
mod structs;
mod ui;
mod window;
mod world;

use bevy::prelude::*;
use structs::plugins::{
    AssetPlugin, CameraPlugin, EnemyPlugin, InputPlugin, MovementPlugin, PlayerPlugin, StatePlugin,
    UiPlugin, WindowInfoPlugin, WorldPlugin,
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
            StatePlugin,
            MovementPlugin,
        ))
        .run();
}
