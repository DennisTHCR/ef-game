use bevy::prelude::*;

use crate::structs::{
    mobs::{Acceleration, Velocity},
    plugins::MovementPlugin,
    state::GameState,
};

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (apply_acceleration, apply_velocity)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn apply_velocity(mut entities: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    entities.iter_mut().for_each(|(mut transform, velocity)| {
        transform.translation += velocity.0.extend(0.0) * time.delta_seconds();
    });
}

fn apply_acceleration(mut entities: Query<(&mut Velocity, &mut Acceleration)>, time: Res<Time>) {
    entities
        .iter_mut()
        .for_each(|(mut velocity, mut acceleration)| {
            velocity.0 += acceleration.0 * time.delta_seconds();
            velocity.0 = velocity.0.clamp_length(0., 80.);
            acceleration.0 = Vec2::ZERO;
        });
}
