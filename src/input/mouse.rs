use bevy::{
    input::mouse::MouseWheel,
    prelude::*,
};

use crate::structs::{input::MouseInput, plugins::MousePlugin};

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init)
            .add_systems(Update, update);
    }
}

fn init(mut commands: Commands) {
    commands.insert_resource(MouseInput::default());
}

fn update(mut scroll: EventReader<MouseWheel>, mut mouse_input: ResMut<MouseInput>) {
    mouse_input.scroll_up = false;
    mouse_input.scroll_down = false;
    for event in scroll.read() {
        mouse_input.scroll_up = event.y < 0.;
        mouse_input.scroll_down = event.y > 0.;
    }
    scroll.clear();
}
