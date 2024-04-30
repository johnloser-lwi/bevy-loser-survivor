
use bevy::prelude::*;
use crate::audio::events::RequestGlobalAudioEvent;
use crate::game::character::components::Health;
use crate::game::events::{OnLevelUp};
use crate::game::player::components::Player;
use crate::states::{AppState, GameState};
use crate::game::resources::*;


pub fn check_player_dead (
    player_query: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<AppState>>,
    sounds: Res<Sounds>,
    mut request_global_audio_event: EventWriter<RequestGlobalAudioEvent>
) {
    if let Ok(health) = player_query.get_single() {
        if health.is_dead() {

            request_global_audio_event.send(RequestGlobalAudioEvent {
                sound: sounds.player_die.clone(),
                is_loop: false
            });

            next_state.set(AppState::GameOver);
        }
    }
}

pub fn load_textures (
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.insert_resource(Textures {
        zombie: asset_server.load("sprites/zombie.png"),
        coin: asset_server.load("sprites/coin.png"),
        whip: asset_server.load("sprites/whip.png"),
        fire_ball: asset_server.load("sprites/fire_ball.png"),
        force_field: asset_server.load("sprites/force_field.png")
    });
}

pub fn load_sounds (
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.insert_resource(Sounds {
        whip: asset_server.load("sounds/temp.ogg"),
        coin: asset_server.load("sounds/temp.ogg"),
        fire_ball: asset_server.load("sounds/temp.ogg"),
        force_field: asset_server.load("sounds/temp.ogg"),
        enemy_damage: asset_server.load("sounds/temp.ogg"),
        player_damage: asset_server.load("sounds/temp.ogg"),
        player_die: asset_server.load("sounds/temp.ogg"),
        level_up: asset_server.load("sounds/temp.ogg"),
        game_over: asset_server.load("sounds/temp.ogg"),
        music: asset_server.load("sounds/temp.ogg")
    });
}

pub fn unload_sounds (
    mut commands: Commands
) {
    commands.remove_resource::<Sounds>();
}

pub fn unload_textures (
    mut commands: Commands
) {
    commands.remove_resource::<Textures>();
}


pub fn switch_upgrade_state (
    mut level_up_event: EventReader<OnLevelUp>,
    mut next_state: ResMut<NextState<GameState>>
) {
    for _ in level_up_event.read() {
        next_state.set(GameState::Upgrade);
    }
}



pub fn pause_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    game_state: Res<State<GameState>>
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {

        let state = game_state.get();

        if *state == GameState::Running {
            next_state.set(GameState::Paused);
        }
        else if *state == GameState::Paused {
            next_state.set(GameState::Running);
        }
    }
}

