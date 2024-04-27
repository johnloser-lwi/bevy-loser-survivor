use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, OnEnter};
use crate::game::gamplay::systems::{insert_gameplay_data, remove_gameplay_data};
use crate::states::AppState;

pub mod resources;
mod systems;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game),
                         (
                             remove_gameplay_data,
                             insert_gameplay_data
                             ).chain()
            )


        ;
    }
}