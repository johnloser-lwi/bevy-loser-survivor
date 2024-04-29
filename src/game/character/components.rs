use bevy::prelude::*;

#[derive(Component)]
pub struct Character {
    pub speed: f32,
    pub direction: Vec2
}

#[derive(Component)]
pub struct DamageFlash {
    pub timer: Timer,
    pub color: Color
}

impl DamageFlash {
    pub fn flash(&mut self, sprite: &mut Sprite) {
        self.timer = Timer::from_seconds(0.1, TimerMode::Once);
        sprite.color = self.color;
    }
}


#[derive(Component)]
pub struct Health {
    pub health: f32,
    pub max_health: f32,
    pub regeneration: f32
}

impl Health {
    pub fn is_dead(&self) -> bool {
        self.health <= 0.0
    }

    pub fn take_damage(&mut self, damage: f32) {
        self.health -= damage;
    }
}