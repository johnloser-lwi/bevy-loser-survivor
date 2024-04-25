pub mod components;
mod systems;
mod resources;

use bevy::prelude::*;
use crate::{AppState, GameState};
use crate::game::enemy::resources::EnemySpawnTimer;
use crate::game::enemy::systems::*;


pub const ENEMY_SPAWN_TIME: f32 = 3.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // resource
            .init_resource::<EnemySpawnTimer>()

            // update
            .add_systems(Update,
                 (
                     spawn_enemy,
                     update_enemy_direction,
                     update_enemy_timer
                 )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running)))

            // game end
            .add_systems(OnExit(AppState::Game), despawn_enemy)
        ;
    }
}