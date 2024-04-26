use bevy::prelude::*;
use crate::game::character::components::{AnimationConfig, Character};

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

pub fn handle_character_animation(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut TextureAtlas, &Character)>,
) {
    for (mut config, mut atlas, character) in query.iter_mut() {
        // we track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());


        if character.direction.length() == 0.0 {

            atlas.index = config.first_sprite_index;

            continue;
        }

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.finished() {
            if atlas.index == config.last_sprite_index {
                // ...and it IS the last frame, then we move back to the first frame and stop.
                atlas.index = config.first_sprite_index;
            } else {
                // ...and it is NOT the last frame, then we move to the next frame...
                atlas.index += 1;
                // ...and reset the frame timer to start counting all over again

            }

            config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
        }


    }
}