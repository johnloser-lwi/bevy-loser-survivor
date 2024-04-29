use bevy::prelude::*;

#[derive(Component)]
pub struct HealthBar{
    pub size: Vec2,
    pub offset: Vec2
}

impl Default for HealthBar {
    fn default() -> Self {
        HealthBar {
            size: Vec2::new(16.0, 3.0),
            offset: Vec2::new(0.0, -20.0)
        }
    }
}