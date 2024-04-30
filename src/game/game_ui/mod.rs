mod layout;
mod interactions;
mod components;

use bevy::prelude::*;
use crate::game::game_ui::interactions::update_xp_ui;
use crate::game::game_ui::layout::{despawn_game_ui, spawn_game_ui};
use crate::states::AppState;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), spawn_game_ui)
            .add_systems(OnExit(AppState::Game), despawn_game_ui)

            .add_systems(Update, update_xp_ui
                .run_if(in_state(AppState::Game))
            )
        ;

    }
}