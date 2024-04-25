use bevy::prelude::*;
use crate::game::player::components::Player;


pub fn spawn_player(
    mut commands: Commands,
) {
    println!("Player spawned!");
}

pub fn despawn_player (
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>
)
{
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}


pub fn handle_player_movement() {

}