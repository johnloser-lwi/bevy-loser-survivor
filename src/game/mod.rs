mod systems;
mod player;
mod character;
mod enemy;
mod camera;


use bevy::prelude::*;



pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                (
                    camera::CameraPlugin,
                    player::PlayerPlugin,
                    enemy::EnemyPlugin,
                    character::CharacterPlugin,
                )
            );
    }
}