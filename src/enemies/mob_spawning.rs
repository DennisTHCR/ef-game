use std::time::Duration;

use bevy::prelude::*;

use crate::structs::{
    assets::TextureHandles,
    markers::{EnemyMarker, SpawnerMarker},
    mobs::{AttackTimer, Health},
    plugins::MobSpawnPlugin,
    state::GameState,
    world::SpawnerTimer,
};

impl Plugin for MobSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_spawners.run_if(in_state(GameState::Playing)));
    }
}

fn tick_spawners(
    mut spawners: Query<(&mut SpawnerTimer, &Transform), With<SpawnerMarker>>,
    time: Res<Time>,
    mut commands: Commands,
    texture_handles: Res<TextureHandles>,
) {
    spawners.iter_mut().for_each(|(mut timer, &transform)| {
        timer.0.tick(Duration::from_secs_f32(time.delta_seconds()));
        if timer.0.just_finished() {
            let mut new_transform = transform;
            new_transform.translation.z = -0.5;
            commands.spawn((
                SpriteBundle {
                    texture: texture_handles.enemy_sprite.clone(),
                    transform: new_transform,
                    ..default()
                },
                EnemyMarker,
                Health::default_mob(),
                AttackTimer::default(),
            ));
        }
    });
}
