use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ParsedInput {
    pub cursor_position: Vec2,
    pub direction: Vec2,
    pub toggle_inventory: bool,
    pub scroll_up: bool,
    pub scroll_down: bool,
    pub left_click: bool,
    pub selected_tool: i32,
}
