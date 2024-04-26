use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::game::character::components::{AnimationConfig, Character};
use crate::game::player::components::Player;
use crate::RENDER_SIZE;


pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {


    let layout = TextureAtlasLayout::from_grid(Vec2::splat(32.0), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_config = AnimationConfig::new(0, 1, 5);

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(RENDER_SIZE.x / 2.0, RENDER_SIZE.y / 2.0, 0.0),
                texture: asset_server.load("sprites/mage.png"),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config.first_sprite_index
            },
            animation_config,
            Character {
                size: 32,
                speed: 100.0,
                direction: Vec2::default()
            },
            Player {},
            Collider::capsule(Vec2::new(0.0, 5.0), Vec2::new(0.0, -5.0), 8.0)
        )
    );


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

pub fn handle_player_input (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Character, With<Player>>
) {
    if let Ok(mut character) = player_query.get_single_mut() {

        let mut direction: Vec2 = Vec2::new(0.0, 0.0);

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y = 1.0;
        }
        else if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y = -1.0;
        }
        else {
            direction.y = 0.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x = -1.0;
        }
        else if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x = 1.0;
        }
        else {
            direction.x = 0.0;
        }

        character.direction = if direction.length() > 0.0 {
            direction.normalize()
        }
        else {
            Vec2::new(0.0, 0.0)
        };
    }
}