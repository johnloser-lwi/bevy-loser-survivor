mod systems;
pub mod events;
pub mod components;

use bevy::prelude::*;
use crate::audio::systems::{handle_global_audio_request, handle_spatial_audio_request, spawn_spatial_listener, update_spatial_audio, update_spatial_listener};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<events::RequestSpatialAudioEvent>()
            .add_event::<events::RequestGlobalAudioEvent>()
            .add_systems(Startup, spawn_spatial_listener)
            .add_systems(Update, (
                handle_spatial_audio_request,
                update_spatial_audio,
                update_spatial_listener,
                handle_global_audio_request
            ))
        ;
    }
}