use bevy::app::{App, FixedUpdate, Plugin};
use bevy::prelude::{in_state, IntoSystemConfigs, Startup};
use crate::game::character::resources::CharacterTextureAtlasLayout;
use crate::game::character::systems::{flip_sprite, handle_character_animation, handle_character_movement, handle_damage_flash, setup_character_atlas, y_sort};
use crate::states::{AppState, GameState};

pub mod components;
mod systems;
pub mod resources;


pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CharacterTextureAtlasLayout>()

            // startup
            .add_systems(Startup, setup_character_atlas)

            .add_systems(FixedUpdate,
                 (
                     flip_sprite,
                     handle_character_movement,
                     handle_character_animation,
                     y_sort,
                     handle_damage_flash
                 )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running))
            )
        ;
    }
}