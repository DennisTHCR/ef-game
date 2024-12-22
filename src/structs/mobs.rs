use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub max_health: i32,
    pub current_health: i32,
    pub regeneration: i32,
}

impl Default for Health {
    fn default() -> Self {
        Health {
            max_health: 100,
            current_health: 100,
            regeneration: 0,
        }
    }
}

impl Health {
    pub fn default_mob() -> Self {
        Health {
            max_health: 2,
            current_health: 2,
            ..default()
        }
    }
}

#[derive(Component)]
pub struct AttackTimer(pub Timer);

impl Default for AttackTimer {
    fn default() -> Self {
        AttackTimer(Timer::from_seconds(2.0, TimerMode::Once))
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
pub struct Velocity(pub Vec2);

#[derive(Component, Default)]
pub struct Acceleration(pub Vec2);
