mod combat;
mod mob_spawning;
mod movement;

use bevy::prelude::*;
use combat::damage_player;
use movement::{forcefield, pathfind};

use crate::structs::{
    markers::EnemyMarker,
    mobs::{DeathTimer, Health, Velocity},
    plugins::{EnemyPlugin, MobSpawnPlugin},
    state::GameState,
};

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                pathfind,
                forcefield,
                damage_player,
                tick_death_timer,
                check_health,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_plugins(MobSpawnPlugin);
    }
}

fn check_health(
    query: Query<(&Health, Entity), (With<EnemyMarker>, Without<DeathTimer>)>,
    mut commands: Commands,
) {
    query.iter().for_each(|(health, entity)| {
        if health.0 <= 0. {
            commands.entity(entity).insert(DeathTimer::default());
        }
    });
}

fn tick_death_timer(
    mut enemies: Query<(&mut DeathTimer, Entity, &mut Sprite), With<EnemyMarker>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    enemies
        .iter_mut()
        .for_each(|(mut death_timer, entity, mut sprite)| {
            death_timer.0.tick(time.delta());
            if death_timer.0.just_finished() {
                commands.entity(entity).remove::<EnemyMarker>();
                commands.entity(entity).remove::<Velocity>();
                sprite.color = Color::srgba(0.2, 0.2, 0.2, 0.2);
            }
        });
}
