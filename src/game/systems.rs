
use bevy::prelude::*;
use crate::game::character::components::Health;
use crate::game::player::components::Player;
use crate::states::AppState;


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