mod whip;

use bevy::prelude::*;
use crate::game::weapons::whip::{insert_whip_data, remove_whip_data, setup_whip_atlas, spawn_whips, update_whips, WhipTextureAtlasLayout};
use crate::states::{AppState, GameState};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WhipTextureAtlasLayout>()
            .add_systems(Startup, setup_whip_atlas)
            .add_systems(OnEnter(AppState::Game),
                         (insert_whip_data)
            )
            .add_systems(OnExit(AppState::Game),
                         (remove_whip_data)
            )

            .add_systems(FixedUpdate,
                     (
                         spawn_whips,
                         update_whips
                     )
                         .run_if(in_state(AppState::Game))
                         .run_if(in_state(GameState::Running))

            )

        ;
    }
}