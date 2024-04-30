use bevy::prelude::*;
use crate::audio::events::RequestGlobalAudioEvent;
use crate::game::events::{OnEnemyDie, OnLevelUp, OnPickupCoin};
use crate::game::gameplay::resources::GameplayData;
use crate::game::resources::Sounds;


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
    mut level_up_event_writer: EventWriter<OnLevelUp>,
    sounds: Res<Sounds>,
    mut global_audio_request: EventWriter<RequestGlobalAudioEvent>
) {
    for evt in pickup_coin_event_reader.read() {
        gameplay_data.xp += evt.xp;
        if gameplay_data.xp == gameplay_data.xp_to_next_level {
            gameplay_data.level += 1;
            gameplay_data.set_xp_to_next_level();
            level_up_event_writer.send(OnLevelUp { });

            global_audio_request.send(RequestGlobalAudioEvent {
                sound: sounds.level_up.clone(),
                is_loop: false
            });

            println!("Level : {}", gameplay_data.level);
        }
    }
}

pub fn update_game_time(
    time: Res<Time>,
    mut gameplay_data: ResMut<GameplayData>
) {
    gameplay_data.game_time += time.delta_seconds();
}