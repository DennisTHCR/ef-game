use bevy::prelude::*;
use crate::structs::{assets::MeshHandles, plugins::AssetPlugin};

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_meshes);
    }
}

fn init_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(MeshHandles {
        health_rectangle: meshes.add(Rectangle::new(100.0, 20.0))
    });
}