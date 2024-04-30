use bevy::prelude::*;

use crate::AppState;
use crate::states::GameState;


pub fn load_game(
    mut next_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    game_state.set(GameState::Running);
    next_state.set(AppState::Game);
}