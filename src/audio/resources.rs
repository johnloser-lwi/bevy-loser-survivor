use bevy::prelude::*;

#[derive(Resource)]
pub struct AudioCooldownList {
    pub list: Vec<(AssetId<AudioSource>, f32)>
}

impl Default for AudioCooldownList {
    fn default() -> Self {
        AudioCooldownList {
            list: Vec::new()
        }
    }
}