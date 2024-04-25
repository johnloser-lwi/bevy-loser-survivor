use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game::player::components::Player;


pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {

    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

pub fn reset_camera(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<&mut Transform, With<Camera>>
) {
    let window = window_query.get_single().unwrap();

    if let Ok(mut camera_tranform) = camera_query.get_single_mut() {
        camera_tranform.translation.x = window.width() / 2.0;
        camera_tranform.translation.y = window.height() / 2.0;
    }
}

pub fn camera_follow_player(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {

            camera_transform.translation = player_transform.translation;

        }
    }
}