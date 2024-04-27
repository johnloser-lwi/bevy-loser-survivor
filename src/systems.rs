use bevy::app::AppExit;
use bevy::prelude::*;

use crate::AppState;

// global exit
pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_write: EventWriter<AppExit>
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit_event_write.send(AppExit);
    }
}

pub fn load_game(
    mut next_state: ResMut<NextState<AppState>>
) {
    next_state.set(AppState::Game);
}