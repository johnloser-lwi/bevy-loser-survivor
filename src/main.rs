
use bevy::prelude::*;

fn main() {
    App::new()
    .insert_state(AppState::Game)
    .insert_state(GameState::Running)
    .add_plugins(DefaultPlugins)
    .run();
}


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused
}
