use bevy::prelude::*;
use crate::game::enemy::components::Enemy;


pub fn spawn_enemy(
    mut commands: Commands
) {
    println!("Spawn enemy!");
}

pub fn despawn_enemy(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>
) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

pub fn update_enemy_movement(
) {

}