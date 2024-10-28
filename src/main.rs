mod player;
mod camera;
mod input;

use bevy::prelude::*;
use camera::CameraPlugin;
use player::PlayerPlugin;
use input::InputPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
        )
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(InputPlugin)
        .run();
}
