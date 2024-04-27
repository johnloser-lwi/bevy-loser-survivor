pub mod components;
mod systems;
mod resources;

use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;
use crate::{AppState, GameState};
use crate::game::enemy::resources::{EnemyConfigurations, EnemySpawnTimer};
use crate::game::enemy::systems::*;


pub const ENEMY_SPAWN_TIME: f32 = 3.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // resource
            .init_resource::<EnemySpawnTimer>()
            .init_resource::<EnemyConfigurations>()

            // update
            .add_systems(FixedUpdate,
                 (
                     spawn_enemy,
                     update_enemy_direction,
                     update_enemy_timer,
                     damage_player,
                     check_enemy_health
                 )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running)))

            // game end
            .add_systems(OnExit(AppState::Game), despawn_enemy)
        ;
    }
}

pub struct EnemyConfig {
    texture: String,
    speed: f32,
    health: f32,
    size: usize,
    damage: f32,
    collider: Collider
}