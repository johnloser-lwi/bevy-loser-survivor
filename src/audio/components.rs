use bevy::prelude::*;

#[derive(Component)]
pub struct SpatialEmitter ;

#[derive(Component)]
pub struct AudioDespawnTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct AudioListener;

#[derive(Component)]
pub struct LoopAudio {
    pub name: String
}