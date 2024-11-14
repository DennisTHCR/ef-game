use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CursorPos(pub Vec2);

#[derive(Resource, Default)]
pub struct KeyboardInput {
    pub direction: Vec2,
}

#[derive(Resource, Default)]
pub struct MouseInput {
    pub scroll_up: bool,
    pub scroll_down: bool,
    pub left_click: bool,
}
