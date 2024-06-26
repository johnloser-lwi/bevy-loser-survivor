pub mod whip;
pub mod force_field;
pub mod fire_ball;

use bevy::prelude::*;
use crate::game::weapons::fire_ball::{insert_fire_ball_data, remove_fire_ball_data, spawn_fire_ball, update_fire_ball};
use crate::game::weapons::force_field::{insert_force_field_data, remove_force_field_data, spawn_force_field, update_force_field};
use crate::game::weapons::whip::{insert_whip_data, remove_whip_data, setup_whip_atlas, spawn_whips, update_whips, WhipTextureAtlasLayout};
use crate::states::{AppState, GameState};

use self::fire_ball::despawn_fire_ball;
use self::force_field::despawn_force_field;
use self::whip::despawn_whip;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WhipTextureAtlasLayout>()
            .add_systems(Startup, setup_whip_atlas)
            .add_systems(OnEnter(AppState::Loading),
                         (
                             insert_whip_data,
                             insert_force_field_data,
                             insert_fire_ball_data
                         )
            )
            .add_systems(OnExit(AppState::Game),
                         (
                             remove_whip_data,
                             remove_force_field_data,
                             remove_fire_ball_data,
                             despawn_fire_ball,
                             despawn_force_field,
                             despawn_whip
                         )
            )

            .add_systems(FixedUpdate,
                     (
                         spawn_whips,
                         spawn_force_field,
                         spawn_fire_ball,
                         update_whips,
                         update_force_field,
                         update_fire_ball,
                     )
                         .run_if(in_state(AppState::Game))
                         .run_if(in_state(GameState::Running))

            )

        ;
    }
}

pub struct WeaponData {
    pub level: u32,
    pub count: u32,
    pub damage: f32,
    pub cooldown: f32,
    pub timer: Vec<Timer>
}

impl WeaponData {

    pub fn add_timer(&mut self) {
        self.count += 1;
        self.timer.push(Timer::from_seconds(self.cooldown, TimerMode::Once));
    }

    pub fn reset_timer(&mut self, index: usize) {

        if index >= self.timer.len() {
            return;
        }

        self.timer[index] = Timer::from_seconds(self.cooldown, TimerMode::Once);
    }
}