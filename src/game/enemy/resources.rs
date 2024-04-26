use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;
use crate::game::enemy::{ENEMY_SPAWN_TIME, EnemyConfig};

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer { timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Once) }
    }
}

#[derive(Resource)]
pub struct EnemyConfigurations {
    pub configs: Vec<EnemyConfig>
}

impl Default for EnemyConfigurations {
    fn default() -> Self {
        EnemyConfigurations {
            configs: vec![
                EnemyConfig {
                    speed: 20.0,
                    health: 5.0,
                    damage:5.0,
                    size: 32,
                    texture: String::from("sprites/zombie.png"),
                    collider: Collider::capsule(Vec2::new(0.0, -5.0), Vec2::new(0.0, 5.0), 8.0)
                }
            ]
        }
    }
}