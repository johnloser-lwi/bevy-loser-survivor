
use bevy::prelude::*;
use crate::game::character::components::Health;
use crate::game::player::components::Player;
use crate::states::AppState;
use crate::game::resources::*;


pub fn check_player_dead (
    player_query: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<AppState>>
) {
    if let Ok(health) = player_query.get_single() {
        if health.is_dead() {
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
        whip: asset_server.load("sprites/whip.png")
    });
}

pub fn unload_textures (
    mut commands: Commands
) {
    commands.remove_resource::<Textures>();
}