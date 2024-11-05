use bevy::prelude::Resource;

#[derive(Resource)]
pub struct CameraSettings {
    pub scale: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        CameraSettings { scale: 0.3 }
    }
}
