use bevy::prelude::*;
use crate::game::enemy::ENEMY_SPAWN_TIME;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer { timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Once) }
    }
}