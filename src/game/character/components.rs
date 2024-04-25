use bevy::prelude::*;

#[derive(Component)]
pub struct Character {
    pub speed: f32,
    pub size: usize,
    pub direction: Vec2
}