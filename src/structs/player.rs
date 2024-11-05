use bevy::prelude::Resource;

#[derive(Resource)]
pub struct PlayerSettings {
    pub speed: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        PlayerSettings { speed: 100. }
    }
}
