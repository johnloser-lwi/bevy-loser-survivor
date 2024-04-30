use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub damage: f32,
    pub attack_timer: Timer
}