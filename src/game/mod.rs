mod systems;
mod player;
mod character;
mod enemy;
mod camera;
mod gamplay;
mod weapons;
mod animation;
mod events;
mod coin;


use bevy::prelude::*;
use crate::game::events::{OnEnemyDie, OnLevelUp, OnPickupCoin};
use crate::game::systems::check_player_dead;
use crate::states::{AppState, GameState};


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
                    gamplay::GamePlayPlugin,
                    weapons::WeaponPlugin,
                    coin::CoinPlugin
                )
            )

            .add_event::<OnEnemyDie>()
            .add_event::<OnPickupCoin>()
            .add_event::<OnLevelUp>()

            .add_systems(Update, check_player_dead
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running))
            )


        ;
    }
}