use bevy::prelude::*;

#[derive(Resource)]
pub struct MeshHandles {
    pub health_rectangle: Handle<Mesh>
}