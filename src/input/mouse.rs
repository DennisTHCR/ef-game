use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::structs::{input::MouseInput, plugins::MousePlugin};

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init).add_systems(Update, (read_mouse_wheel, read_mouse_clicks));
    }
}

fn init(mut commands: Commands) {
    commands.insert_resource(MouseInput::default());
}

fn read_mouse_wheel(mut scroll: EventReader<MouseWheel>, mut mouse_input: ResMut<MouseInput>) {
    mouse_input.scroll_up = false;
    mouse_input.scroll_down = false;
    for event in scroll.read() {
        mouse_input.scroll_up = event.y < 0.;
        mouse_input.scroll_down = event.y > 0.;
    }
    scroll.clear();
}

fn read_mouse_clicks(clicks: Res<ButtonInput<MouseButton>>, mut mouse_input: ResMut<MouseInput>) {
    mouse_input.left_click = clicks.just_pressed(MouseButton::Left);
}