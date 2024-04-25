pub mod components;
mod systems;

use bevy::prelude::*;
use crate::{AppState, GameState};
use crate::game::enemy::systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // update
            .add_systems(Update, (spawn_enemy, update_enemy_movement)
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running)))

            // game end
            .add_systems(OnExit(AppState::Game), despawn_enemy)
        ;
    }
}