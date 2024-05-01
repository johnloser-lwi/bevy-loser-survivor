pub mod resources;

use bevy::prelude::*;

pub struct GamepadPlugin;

impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(resources::GamepadInput {
                axis: Vec2::ZERO
            })
        ;
    }
}