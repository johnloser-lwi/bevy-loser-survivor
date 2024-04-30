mod layout;
mod interactions;
mod components;

use bevy::prelude::*;
use crate::game_over_ui::interactions::handle_main_menu_button;
use crate::game_over_ui::layout::{spawn_game_over_ui, despawn_game_over_ui};
use crate::states::AppState;

pub struct GameOverUiPlugin;

impl Plugin for GameOverUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::GameOver), spawn_game_over_ui)
            .add_systems(OnExit(AppState::GameOver), layout::despawn_game_over_ui)
            .add_systems(Update, handle_main_menu_button
                .run_if(in_state(AppState::GameOver))
            )
        ;
    }
}