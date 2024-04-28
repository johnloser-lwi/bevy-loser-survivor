use bevy::prelude::*;
use crate::game::events::{OnEnemyDie, OnLevelUp, OnPickupCoin};
use crate::game::gameplay::resources::GameplayData;


pub fn insert_gameplay_data(
    mut commands: Commands
) {
    commands.insert_resource(GameplayData::default());
}

pub fn remove_gameplay_data(
    mut commands: Commands
) {
    commands.remove_resource::<GameplayData>();
}

pub fn update_head_count(
    mut enemy_die_event_reader: EventReader<OnEnemyDie>,
    mut gameplay_data: ResMut<GameplayData>
) {
    for _ in enemy_die_event_reader.read(){
        gameplay_data.head_count += 1;
    }
}

pub fn update_xp(
    mut pickup_coin_event_reader: EventReader<OnPickupCoin>,
    mut gameplay_data: ResMut<GameplayData>,
    mut level_up_event_writer: EventWriter<OnLevelUp>
) {
    for evt in pickup_coin_event_reader.read() {
        gameplay_data.xp += evt.xp;
        if gameplay_data.xp == gameplay_data.xp_to_next_level {
            gameplay_data.level += 1;
            gameplay_data.set_xp_to_next_level();
            level_up_event_writer.send(OnLevelUp { });
            println!("Level : {}", gameplay_data.level);
        }
    }
}