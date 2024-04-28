mod layout;
mod interactions;
pub mod components;

use bevy::prelude::*;
use crate::game::upgrade_menu::interactions::level_up_button_system;
use crate::game::upgrade_menu::layout::{despawn_level_up_ui, spawn_level_up_ui};
use crate::states::{AppState, GameState};

pub struct LevelUpUIPlugin;

impl Plugin for LevelUpUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Upgrade), spawn_level_up_ui)
            .add_systems(OnExit(GameState::Upgrade), despawn_level_up_ui)

            .add_systems(Update,
                level_up_button_system
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Upgrade))
            )
        ;
    }
}