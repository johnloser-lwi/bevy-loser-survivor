mod systems;
mod player;
mod character;
mod enemy;


use bevy::prelude::*;



pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((player::PlayerPlugin, enemy::EnemyPlugin));
    }
}