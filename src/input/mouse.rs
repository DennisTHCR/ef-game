use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::structs::{input::ParsedInput, plugins::MousePlugin};

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (read_mouse_wheel, read_mouse_clicks));
    }
}

fn read_mouse_wheel(mut scroll: EventReader<MouseWheel>, mut parsed_input: ResMut<ParsedInput>) {
    parsed_input.scroll_up = false;
    parsed_input.scroll_down = false;
    for event in scroll.read() {
        parsed_input.scroll_up = event.y < 0.;
        parsed_input.scroll_down = event.y > 0.;
    }
    scroll.clear();
}

fn read_mouse_clicks(clicks: Res<ButtonInput<MouseButton>>, mut mouse_input: ResMut<ParsedInput>) {
    mouse_input.left_click = clicks.just_pressed(MouseButton::Left);
}
