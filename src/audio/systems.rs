use bevy::audio::Volume;
use bevy::prelude::*;
use crate::audio::components::{AudioDespawnTimer, AudioListener, LoopAudio, SpatialEmitter};
use crate::audio::events::{RequestGlobalAudioEvent, RequestSpatialAudioEvent, RequestStopMusicEvent};
use crate::audio::resources::AudioCooldownList;

pub fn spawn_spatial_listener(
    mut commands: Commands
) {
    let listener = SpatialListener::new(200.0);

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

        let audio_bundle = AudioBundle {
            source: evt.sound.clone(),
            settings: playback_setting,
        };
        if evt.is_loop {
            commands.spawn((audio_bundle, LoopAudio { name: String::from("music")}));
        }
        else {
            commands.spawn(
                (
                    audio_bundle,
                    AudioDespawnTimer {
                        timer: Timer::from_seconds(3.0, TimerMode::Once),
                    }
                )
            );
        }
    }
}

pub fn handle_audio_cooldown (
    mut audio_cooldown_list: ResMut<AudioCooldownList>,
    time: Res<Time>
) {
    for (_, cd) in audio_cooldown_list.list.iter_mut() {
        *cd -= time.delta_seconds();
    }
}

pub fn handle_spatial_audio_request (
    mut commands: Commands,
    mut spatial_audio_request: EventReader<RequestSpatialAudioEvent>,
    mut audio_cooldown_list: ResMut<AudioCooldownList>,
    camera_query: Query<&Transform, With<Camera2d>>
) {
    for evt in spatial_audio_request.read() {

        let mut settings = PlaybackSettings::ONCE.with_spatial(true);
        settings.volume = Volume::new(0.4);

        let mut has_cooldown = false;
        let mut can_play = true;
        for (handle, cd) in audio_cooldown_list.list.iter_mut() {
            if *handle == evt.sound.id()  {
                has_cooldown = true;
                if  *cd > 0.0 {
                    can_play = false;
                }
                else {
                    *cd = 0.2;
                }
                break;
            }
        }

        if !can_play { continue; }

        if !has_cooldown { audio_cooldown_list.list.push((evt.sound.id(), 0.2));}

        let cam_transform = camera_query.get_single().unwrap();

        let pos_x  = if evt.position.x < cam_transform.translation.x {
            (cam_transform.translation.x - evt.position.x) + cam_transform.translation.x
        }
        else if evt.position.x > cam_transform.translation.x {
            cam_transform.translation.x - (evt.position.x - cam_transform.translation.x)
        }
        else {
            evt.position.x
        };

        commands.spawn(
            (
                SpatialBundle {
                    transform: Transform::from_xyz(pos_x, evt.position.y, 0.0),
                    ..default()
                },
                AudioBundle {
                    source: evt.sound.clone(),
                    settings,
                },
                SpatialEmitter,
                AudioDespawnTimer {
                    timer: Timer::from_seconds(3.0, TimerMode::Once),
                }
            )
        );
    }
}

pub fn handle_audio_despawn(
    mut commands: Commands,
    mut audio_query: Query<(Entity, &mut AudioDespawnTimer)>,
    time: Res<Time>
) {
    for (entity, mut timer) in audio_query.iter_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn handle_stop_music(
    mut commands: Commands,
    audio_query: Query<(Entity, &LoopAudio)>,
    mut request_stop_music_event: EventReader<RequestStopMusicEvent>
) {
    for _ in request_stop_music_event.read() {
        for (entity, loop_audio) in audio_query.iter() {
            if loop_audio.name != "music" { continue; }

            commands.entity(entity).despawn_recursive();
        }
    }
}