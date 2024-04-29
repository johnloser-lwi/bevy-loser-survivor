use bevy::prelude::*;
use crate::game::animation::components::AnimationConfig;
use crate::game::character::components::{Character, DamageFlash};
use crate::game::character::resources::CharacterTextureAtlasLayout;
use crate::RENDER_SIZE;

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

        transform.translation.x = transform.translation.x.clamp(-RENDER_SIZE.x, RENDER_SIZE.x);
        transform.translation.y = transform.translation.y.clamp(-RENDER_SIZE.y, RENDER_SIZE.y);
    }
}

pub fn setup_character_atlas(
    mut atlas: ResMut<CharacterTextureAtlasLayout>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(Vec2::splat(32.0), 2, 1, None, None);
    atlas.handle = texture_atlas_layouts.add(layout);
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

pub fn y_sort(
    mut query: Query<&mut Transform, With<Character>>
) {
    for mut transform in query.iter_mut() {
        transform.translation.z = -(1.0f32 / (1.0f32 + (2.0f32.powf(-0.01*transform.translation.y))));
    }
}

pub fn handle_damage_flash (
    time: Res<Time>,
    mut query: Query<(&mut DamageFlash, &mut Sprite)>,
) {
    for (mut flash, mut sprite) in query.iter_mut() {
        flash.timer.tick(time.delta());
        if flash.timer.just_finished() {
            sprite.color = Color::WHITE;
        }
    }
}
