use bevy::prelude::*;


pub struct RequestSpatialAudioEvent {
    pub position: Vec2,
    pub sound: Handle<AudioSource>
}

impl Event for RequestSpatialAudioEvent {

}

pub struct RequestGlobalAudioEvent {
    pub sound: Handle<AudioSource>,
    pub is_loop: bool
}

impl Event for RequestGlobalAudioEvent {

}