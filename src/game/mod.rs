mod systems;
mod player;
mod character;
mod enemy;


use bevy::prelude::*;



pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, _app: &mut App) {
        _app
            .add_plugins((player::PlayerPlugin, enemy::EnemyPlugin));
    }
}