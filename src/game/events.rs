use bevy::prelude::*;

pub struct OnEnemyDie {
    pub position: Vec2
}

impl Event for OnEnemyDie {
}

pub struct OnPickupCoin {
    pub xp: u32
}

impl Event for OnPickupCoin {

}

pub struct OnLevelUp {
}

impl Event for OnLevelUp {
    
}