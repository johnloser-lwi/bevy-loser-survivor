use bevy::prelude::*;
use crate::game::gamplay::resources::GameplayData;


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