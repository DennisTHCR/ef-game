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
            max_health: 10,
            current_health: 10,
            ..default()
        }
    }
}
