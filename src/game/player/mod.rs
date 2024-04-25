mod systems;
pub mod components;

use bevy::prelude::*;
use crate::{AppState, GameState};
use crate::game::player::systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, _app: &mut App) {
        _app
            // enter game
            .add_systems(OnEnter(AppState::Game), (spawn_player))

            // update
            .add_systems(Update, (handle_player_movement).run_if(in_state(AppState::Game)).run_if(in_state(GameState::Running)))

            // exit game
            .add_systems(OnExit(AppState::Game), (despawn_player))

        ;


    }
}