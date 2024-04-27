use bevy::prelude::*;

#[derive(Resource)]
pub struct Textures {
    pub zombie: Handle<Image>,
    pub coin: Handle<Image>,
    pub whip: Handle<Image>
}

impl Default for Textures {
    fn default() -> Self {
        Textures {
            zombie: Handle::default(),
            coin: Handle::default(),
            whip: Handle::default()
        }
    }
}