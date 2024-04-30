mod systems;
mod player;
mod character;
mod enemy;
mod camera;
pub mod gameplay;
mod weapons;
mod animation;
mod events;
mod coin;
pub mod resources;
mod upgrade_menu;
mod health_bar;
mod background;
mod game_ui;

use bevy::prelude::*;
use crate::game::events::{OnEnemyDie, OnLevelUp, OnPickupCoin};
use crate::game::systems::check_player_dead;
use crate::states::{AppState, GameState};

use self::systems::*;


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
                    gameplay::GamePlayPlugin,
                    weapons::WeaponPlugin,
                    coin::CoinPlugin,
                    upgrade_menu::LevelUpUIPlugin,
                    health_bar::HealthBarPlugin,
                    background::BackgroundPlugin,
                    game_ui::GameUiPlugin,
                )
            )

            .add_event::<OnEnemyDie>()
            .add_event::<OnPickupCoin>()
            .add_event::<OnLevelUp>()

            .add_systems(Startup, load_sounds)

            .add_systems(OnEnter(AppState::Loading), load_textures)
            .add_systems(OnExit(AppState::Game), (unload_textures, unload_sounds))

            .add_systems(Update,
                         (
                             check_player_dead,
                             switch_upgrade_state
                         )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Running))
            )

            .add_systems(Update, pause_game
                .run_if(in_state(AppState::Game))
            )
        ;
    }
}