use bevy::input::gamepad::{GamepadAxisChangedEvent, GamepadButtonChangedEvent};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::game::animation::components::AnimationConfig;
use crate::game::character::components::{Character, DamageFlash, Health};
use crate::game::character::resources::CharacterTextureAtlasLayout;
use crate::game::gamepad::resources::GamepadInput;
use crate::game::health_bar::components::HealthBar;
use crate::game::player::components::Player;
use crate::RENDER_SIZE;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<CharacterTextureAtlasLayout>
) {


    let animation_config = AnimationConfig::new(0, 1, 5);

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(RENDER_SIZE.x / 2.0, RENDER_SIZE.y / 2.0, 0.0),
                texture: asset_server.load("sprites/mage.png"),
                ..default()
            },
            TextureAtlas {
                layout: atlas_layout.handle.clone(),
                index: animation_config.first_sprite_index
            },
            animation_config,
            Character {
                speed: 100.0,
                direction: Vec2::default()
            },
            DamageFlash {
                timer: Timer::from_seconds(0.1, TimerMode::Once),
                color: Color::RED
            },
            Player {},
            Health {
                health: 100.0,
                max_health: 100.0,
                regeneration: 0.1
            },
            HealthBar ::default(),
            Collider::capsule(Vec2::new(0.0, 5.0), Vec2::new(0.0, -5.0), 8.0)
        )
    );


}

pub fn despawn_player (
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>
)
{
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn_recursive();
    }
}

pub fn handle_player_input (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut gamepad_axis: EventReader<GamepadAxisChangedEvent>,
    mut gamepad_button: EventReader<GamepadButtonChangedEvent>,
    mut player_query: Query<&mut Character, With<Player>>,
    mut gamepad: ResMut<GamepadInput>
) {
    if let Ok(mut character) = player_query.get_single_mut() {

        let mut direction: Vec2 = Vec2::new(0.0, 0.0);

        let mut has_gamepad_input = false;

        let mut has_dpad_input = false;

        for evt in gamepad_button.read() {
            if evt.gamepad.id != 0 {
                continue;
            }

            if evt.button_type == GamepadButtonType::DPadDown {
                gamepad.axis.y = -evt.value;
            }
            if evt.button_type == GamepadButtonType::DPadUp {
                gamepad.axis.y = evt.value;
            }
            if evt.button_type == GamepadButtonType::DPadLeft {
                gamepad.axis.x = -evt.value;
            }
            if evt.button_type == GamepadButtonType::DPadRight {
                gamepad.axis.x = evt.value;
            }

            has_dpad_input = true;

        }

        for evt in gamepad_axis.read() {

            if evt.gamepad.id != 0 || has_dpad_input {
                continue;
            }

            if evt.axis_type == GamepadAxisType::LeftStickX {
                gamepad.axis.x = evt.value;
            }
            if evt.axis_type == GamepadAxisType::LeftStickY {
                gamepad.axis.y = evt.value;
            }
        }



        if gamepad.axis.length() > 0.0 {
            has_gamepad_input = true;
            direction = gamepad.axis;
        }


        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y = 1.0;
        }
        else if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y = -1.0;
        }
        else if !has_gamepad_input {
            direction.y = 0.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x = -1.0;
        }
        else if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x = 1.0;
        }
        else if !has_gamepad_input {
            direction.x = 0.0;
        }

        character.direction = if direction.length() > 0.0 {
            direction.normalize()
        }
        else {
            Vec2::new(0.0, 0.0)
        };
    }
}

pub fn health_regeneration (
    mut query: Query<&mut Health, With<Player>>,
    time: Res<Time>
) {
    if let Ok(mut health) = query.get_single_mut() {
        health.health += health.regeneration * time.delta_seconds();
        if health.health > health.max_health {
            health.health = health.max_health;
        }
    }
}