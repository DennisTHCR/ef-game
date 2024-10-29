use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CursorCoords(pub Vec2);

#[derive(Resource, Default)]
pub struct KeyboardInput {
    pub direction: Vec2,
}
