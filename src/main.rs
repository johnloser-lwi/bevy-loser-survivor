
use bevy::prelude::*;


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
    

    // Custom Plugins
    .add_plugins(GamePlugin)


    // Systems
    .add_systems(Update, exit_game)

    .run();
}


