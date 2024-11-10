use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct WindowInfo {
    pub corners: WindowCorners,
}

#[derive(Default)]
pub struct WindowCorners {
    pub top_left: Vec2,
    pub top_right: Vec2,
    pub bottom_left: Vec2,
    pub bottom_right: Vec2,
}
