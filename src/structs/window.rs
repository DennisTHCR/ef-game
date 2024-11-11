use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct WindowInfo {
    pub corner_coords: CornerCoords,
}

#[derive(Default, Clone)]
pub struct CornerCoords {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}
