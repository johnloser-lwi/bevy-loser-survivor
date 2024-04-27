use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::game::animation::components::AnimationConfig;
use crate::game::character::components::{ Character, Health};
use crate::game::character::resources::CharacterTextureAtlasLayout;
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
            Player {},
            Health {
                health: 100.0
            },
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
    mut player_query: Query<&mut Character, With<Player>>
) {
    if let Ok(mut character) = player_query.get_single_mut() {

        let mut direction: Vec2 = Vec2::new(0.0, 0.0);

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y = 1.0;
        }
        else if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y = -1.0;
        }
        else {
            direction.y = 0.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x = -1.0;
        }
        else if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x = 1.0;
        }
        else {
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