use bevy::prelude::*;

use crate::structs::{plugins::StatePlugin, state::GameState};

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::Playing);
    }
}
