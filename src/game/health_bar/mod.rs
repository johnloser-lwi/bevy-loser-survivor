mod systems;
pub mod components;


use bevy::prelude::*;
use crate::game::health_bar::systems::{draw_health_bar, setup_gizmo_config};
use crate::states::{AppState, GameState};

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_gizmo_config)
            .add_systems(Update,
            (
                draw_health_bar
            )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running))
        );
    }
}