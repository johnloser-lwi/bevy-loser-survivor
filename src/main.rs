
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


use game::*;
use crate::states::*;
use crate::systems::exit_game;


mod game;
mod systems;
mod states;


fn main() {
    App::new()
    .insert_state(AppState::Game)
    .insert_state(GameState::Running)
    .add_plugins(DefaultPlugins)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
    .insert_resource(RapierConfiguration {
        gravity: Vec2::ZERO,
        ..default()
    })
    .add_plugins(RapierDebugRenderPlugin::default())
    

    // Custom Plugins
    .add_plugins(GamePlugin)


    // Systems
    .add_systems(Update, exit_game)

    .run();
}


