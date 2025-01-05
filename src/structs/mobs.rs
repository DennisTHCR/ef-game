use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct AttackTimer(pub Timer);

impl Default for AttackTimer {
    fn default() -> Self {
        AttackTimer(Timer::from_seconds(0.8, TimerMode::Once))
    }
}

#[derive(Component)]
pub struct DeathTimer(pub Timer);

impl Default for DeathTimer {
    fn default() -> Self {
        DeathTimer(Timer::from_seconds(0.2, TimerMode::Once))
    }
}

#[derive(Resource)]
pub struct EnemiesAlive(pub i32);

#[derive(Component, Default)]
pub struct Velocity {
    pub current: Vec2,
    pub max: f32,
}

#[derive(Component, Default)]
pub struct Acceleration(pub Vec2);
