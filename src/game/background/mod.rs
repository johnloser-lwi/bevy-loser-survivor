mod systems;
mod components;

use bevy::prelude::*;
use crate::states::AppState;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), systems::spawn_background)
            .add_systems(OnExit(AppState::Game), systems::despawn_background)
        ;
    }
}