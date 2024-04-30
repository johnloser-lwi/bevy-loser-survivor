
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
use bevy_rapier2d::prelude::*;


use game::*;
use systems::load_game;
use crate::states::*;


mod game;
mod systems;
mod states;
mod game_over_ui;
mod styles;
mod main_menu_ui;

pub const RENDER_SIZE: Vec2 = Vec2::new(1280., 720.);
pub const RENDER_SCALE: f32 = 0.4;

fn main() {
    App::new()
    .insert_state(AppState::MainMenu)
    .insert_state(GameState::Running)
    .add_plugins(DefaultPlugins.set(
        WindowPlugin {
            primary_window: Some(Window {
                title: "Losers' World: Zombie Survivor".into(),
                name: Some("bevy.app".into()),
                resolution: (RENDER_SIZE.x, RENDER_SIZE.y).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            ..default()
        }

    ).set(ImagePlugin::default_nearest()))
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
    .insert_resource(RapierConfiguration {
        gravity: Vec2::ZERO,
        ..default()
    })
    //.add_plugins(RapierDebugRenderPlugin::default())
    

    // Custom Plugins
    .add_plugins((
        GamePlugin,
        game_over_ui::GameOverUiPlugin,
        main_menu_ui::MainMenuUIPlugin
    ))


    // Systems
    .add_systems(OnEnter(AppState::Loading), load_game)

    .run();
}


