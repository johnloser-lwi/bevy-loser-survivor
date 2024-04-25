use bevy::prelude::*;
use crate::game::character::components::Character;

pub fn flip_sprite(
    mut sprite_query: Query<(&mut Sprite, &Character), With<Character>>
) {
    for (mut sprite, character) in sprite_query.iter_mut() {
        if character.direction.x < 0.0 {
            sprite.flip_x = true;
        }
        else if character.direction.x > 0.0 {
            sprite.flip_x = false;
        }
    }
}

pub fn handle_character_movement(
    time: Res<Time>,
    mut character_query: Query<(&Character, &mut Transform), With<Character>>
) {
    for (character, mut transform) in character_query.iter_mut() {
        let direction: Vec3 = Vec3::new(character.direction.x, character.direction.y, 0.0);
        transform.translation += direction * (character.speed * time.delta_seconds());
    }
}