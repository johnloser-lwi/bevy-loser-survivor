use bevy::prelude::*;

#[derive(Resource)]
pub struct Textures {
    pub zombie: Handle<Image>,
    pub coin: Handle<Image>,
    pub whip: Handle<Image>,
    pub force_field: Handle<Image>,
    pub fire_ball: Handle<Image>
}

impl Default for Textures {
    fn default() -> Self {
        Textures {
            zombie: Handle::default(),
            coin: Handle::default(),
            whip: Handle::default(),
            force_field: Handle::default(),
            fire_ball: Handle::default()
        }
    }
}

#[derive(Resource)]
pub struct Sounds {
    pub whip: Handle<AudioSource>,
    pub coin: Handle<AudioSource>,
    pub fire_ball: Handle<AudioSource>,
    pub force_field: Handle<AudioSource>,
    pub enemy_damage: Handle<AudioSource>,
    pub player_damage: Handle<AudioSource>,
    pub player_die: Handle<AudioSource>,
    pub level_up: Handle<AudioSource>,
    pub game_over: Handle<AudioSource>,
    pub music: Handle<AudioSource>
}