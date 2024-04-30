mod systems;
pub mod events;
pub mod components;
mod resources;

use bevy::prelude::*;
use crate::audio::resources::AudioCooldownList;
use crate::audio::systems::{handle_global_audio_request, handle_spatial_audio_request, spawn_spatial_listener, handle_audio_despawn, update_spatial_listener, handle_stop_music, handle_audio_cooldown};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AudioCooldownList::default())
            .add_event::<events::RequestSpatialAudioEvent>()
            .add_event::<events::RequestGlobalAudioEvent>()
            .add_event::<events::RequestStopMusicEvent>()
            .add_systems(Startup, spawn_spatial_listener)
            .add_systems(PostUpdate, (
                handle_spatial_audio_request,
                handle_audio_despawn,
                update_spatial_listener,
                handle_global_audio_request,
                handle_stop_music,
                handle_audio_cooldown
            ))
        ;
    }
}