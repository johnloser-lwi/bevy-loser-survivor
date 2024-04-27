use bevy::app::{App, Plugin, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter};
use crate::game::gameplay::systems::{insert_gameplay_data, remove_gameplay_data, update_head_count, update_xp};
use crate::states::{AppState, GameState};

pub mod resources;
mod systems;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game),
                         (
                             remove_gameplay_data,
                             insert_gameplay_data,
                             ).chain()
            )

            .add_systems(Update,
                         (update_head_count, update_xp)

                             .run_if(in_state(AppState::Game))
                             .run_if(in_state(GameState::Running))

            )


        ;
    }
}