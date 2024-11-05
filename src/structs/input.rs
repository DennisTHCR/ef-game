use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CursorCoords(pub Vec2);

#[derive(Resource, Default)]
pub struct KeyboardInput {
    pub direction: Vec2,
}

#[derive(Resource, Default)]
pub struct MouseInput {
    pub scroll_up: bool,
    pub scroll_down: bool,
}
