use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer { timer: Timer::from_seconds(3.0, TimerMode::Once) }
    }
}

pub struct EnemyConfig {
    pub texture: Handle<Image>,
    pub speed: f32,
    pub health: f32,
    pub damage: f32,
    pub collider: Collider
}

#[derive(Resource)]
pub struct EnemyConfigurations {
    pub configs: Vec<EnemyConfig>,
    pub spawn_time: f32,
    pub spawn_count: usize
}

impl Default for EnemyConfigurations {
    fn default() -> Self {
        EnemyConfigurations {
            configs: vec![
                EnemyConfig {
                    speed: 20.0,
                    health: 5.0,
                    damage:5.0,
                    texture: Handle::default(),
                    collider: Collider::capsule(Vec2::new(0.0, -5.0), Vec2::new(0.0, 5.0), 8.0)
                }
            ],
            spawn_time: 3.0,
            spawn_count: 5
        }

    }
}