use std::time::Duration;
use bevy::prelude::*;

#[derive(Component)]
pub struct Character {
    pub speed: f32,
    pub size: usize,
    pub direction: Vec2
}


#[derive(Component)]
pub struct Health {
    pub health: f32
}

impl Health {
    pub fn is_dead(&self) -> bool {
        self.health <= 0.0
    }

    pub fn take_damage(&mut self, damage: f32) {
        self.health -= damage;
    }
}