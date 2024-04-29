use bevy::app::AppExit;
use bevy::prelude::*;

use crate::AppState;
use crate::states::GameState;

// global exit
pub fn pause_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    game_state: Res<State<GameState>>
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {

        let state = game_state.get();

        if *state == GameState::Running {
            next_state.set(GameState::Paused);
        }
        else if *state == GameState::Paused {
            next_state.set(GameState::Running);
        }
    }
}

pub fn load_game(
    mut next_state: ResMut<NextState<AppState>>
) {
    next_state.set(AppState::Game);
}