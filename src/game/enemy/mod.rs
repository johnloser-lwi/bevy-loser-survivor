pub mod components;
mod systems;
mod resources;

use bevy::prelude::*;
use crate::{AppState, GameState};
use crate::game::enemy::resources::*;
use crate::game::enemy::systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // resource
            .init_resource::<EnemySpawnTimer>()

            .add_systems(OnEnter(AppState::Game), setup_enemy_config)

            // update
            .add_systems(FixedUpdate,
                 (
                     spawn_enemy,
                     update_enemy_direction,
                     update_enemy_timer,
                     damage_player,
                     check_enemy_health,
                     cleanup_enemy
                 )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running)))

            // update
            .add_systems(Update,
                         enemy_level_up
                             .run_if(in_state(AppState::Game))
                             .run_if(in_state(GameState::Running)))

            // game end
            .add_systems(OnExit(AppState::Game), despawn_enemy)
        ;
    }
}

