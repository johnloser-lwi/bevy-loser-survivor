mod components;
mod systems;

use bevy::prelude::*;
use crate::game::camera::systems::{camera_follow_player, reset_camera, spawn_camera};
use crate::states::{AppState, GameState};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .add_systems(Startup, spawn_camera)

            // update
            .add_systems(FixedPostUpdate,
                camera_follow_player
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Running)),
            )


            // end game
            .add_systems(OnExit(AppState::Game), reset_camera)
        ;
    }
}