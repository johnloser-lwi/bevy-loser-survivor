use bevy::prelude::*;
use crate::audio::components::{AudioListener, SpatialEmitter};
use crate::audio::events::{RequestGlobalAudioEvent, RequestSpatialAudioEvent};

pub fn spawn_spatial_listener(
    mut commands: Commands
) {
    let listener = SpatialListener::new(100.0);

    commands
        .spawn((SpatialBundle::default(), listener.clone(), AudioListener));
}

pub fn update_spatial_listener(
    mut listener_query: Query<&mut Transform, With<AudioListener>>,
    camera_query: Query<&Transform, (With<Camera2d>, Without<AudioListener>)>,
) {
    if let Ok(mut listener) = listener_query.get_single_mut() {
        if let Ok(camera_transform) = camera_query.get_single() {
            listener.translation = camera_transform.translation;
        }
    }
}

pub fn handle_global_audio_request (
    mut commands: Commands,
    mut audio_request: EventReader<RequestGlobalAudioEvent>,
) {
    for evt in audio_request.read() {

        let playback_setting = if evt.is_loop {
            PlaybackSettings::LOOP
        }
        else {
            PlaybackSettings::ONCE
        };

        commands.spawn(
            (
                AudioBundle {
                    source: evt.sound.clone(),
                    settings: playback_setting,
                },
            )
        );
    }
}

pub fn handle_spatial_audio_request (
    mut commands: Commands,
    mut spatial_audio_request: EventReader<RequestSpatialAudioEvent>,
) {
    for evt in spatial_audio_request.read() {
        commands.spawn(
            (
                SpatialBundle {
                    transform: Transform::from_xyz(evt.position.x, evt.position.y, 0.0),
                    ..default()
                },
                AudioBundle {
                    source: evt.sound.clone(),
                    settings: PlaybackSettings::ONCE.with_spatial(true),
                },
                SpatialEmitter
            )
        );
    }
}

pub fn update_spatial_audio(
    mut commands: Commands,
    audio_query: Query<(Entity, &AudioSink), With<SpatialEmitter>>,
) {
    for (entity, sink) in audio_query.iter() {
        if sink.empty() {
            commands.entity(entity).despawn_recursive();
        }
    }
}