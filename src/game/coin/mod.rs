pub mod components;
mod systems;

use bevy::prelude::*;
use crate::game::coin::systems::{spawn_coin, update_coins};
use crate::states::{AppState, GameState};

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,
                         (
                             spawn_coin,
                             update_coins
                             )
                             .run_if(in_state(AppState::Game))
                             .run_if(in_state(GameState::Running))



            )
        ;
    }
}