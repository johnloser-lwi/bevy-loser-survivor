use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use crate::game::player::components::Player;
use crate::{RENDER_SCALE, RENDER_SIZE};


pub fn spawn_camera(
    mut commands: Commands,
) {
    let mut camera_bundle = Camera2dBundle {
        transform: Transform::from_xyz(RENDER_SIZE.x / 2.0, RENDER_SIZE.y / 2.0, 0.0),
        ..default()
    };

    camera_bundle.projection.scaling_mode = ScalingMode::AutoMax { max_width: RENDER_SIZE.x, max_height:RENDER_SIZE.y };
    camera_bundle.projection.scale = RENDER_SCALE;

    commands.spawn(
        camera_bundle
    );
}

pub fn reset_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>
) {
    if let Ok(mut camera_tranform) = camera_query.get_single_mut() {
        camera_tranform.translation.x = RENDER_SIZE.x / 2.0;
        camera_tranform.translation.y = RENDER_SIZE.y / 2.0;
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