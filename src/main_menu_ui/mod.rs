mod layout;
mod interactions;
mod components;

use bevy::prelude::*;
use crate::states::AppState;

pub struct MainMenuUIPlugin;

impl Plugin for MainMenuUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), layout::spawn_main_menu_ui)
            .add_systems(OnExit(AppState::MainMenu), layout::despawn_main_menu_ui)
            .add_systems(Update, interactions::handle_start_button
                .run_if(in_state(AppState::MainMenu))
            )
        ;
    }
}