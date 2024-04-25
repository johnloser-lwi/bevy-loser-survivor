use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game::character::components::Character;
use crate::game::player::components::Player;


pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture: asset_server.load("sprites/temp.png"),
                ..default()
            },
            Character {
                size: 64,
                speed: 100.0,
                direction: Vec2::default()
            },
            Player {},
            Collider::ball(32.0)
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