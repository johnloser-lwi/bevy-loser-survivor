mod systems;
pub mod components;

use bevy::prelude::*;
use crate::{AppState, GameState};
use crate::game::player::systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // enter game
            .add_systems(OnEnter(AppState::Game), spawn_player)

            // update
            .add_systems(Update,
                     health_regeneration
                     .run_if(in_state(AppState::Game)).run_if(in_state(GameState::Running)))
            .add_systems(Update, handle_player_input
                     .run_if(in_state(AppState::Game)))
            // exit game
            .add_systems(OnExit(AppState::Game), despawn_player)

        ;


    }
}