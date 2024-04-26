use bevy::app::{App, FixedUpdate, Plugin};
use bevy::prelude::{in_state, IntoSystemConfigs};
use crate::game::character::systems::{flip_sprite, handle_character_animation, handle_character_movement};
use crate::states::{AppState, GameState};

pub mod components;
mod systems;


pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(FixedUpdate,
                 (
                     flip_sprite,
                     handle_character_movement,
                     handle_character_animation,
                 )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running))
            )
        ;
    }
}