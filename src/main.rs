
use bevy::prelude::*;
use bevy::app::AppExit;


use game::*;


mod game;


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


// global exit 
fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_write: EventWriter<AppExit>
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit_event_write.send(AppExit);
    }
}
