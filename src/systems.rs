use bevy::prelude::*;

use crate::AppState;


pub fn load_game(
    mut next_state: ResMut<NextState<AppState>>
) {
    next_state.set(AppState::Game);
}