use bevy::prelude::*;
use rand::prelude::*;
use crate::game::character::components::{AnimationConfig, Character};
use crate::game::enemy::components::Enemy;
use crate::game::enemy::ENEMY_SPAWN_TIME;
use crate::game::enemy::resources::EnemySpawnTimer;
use crate::game::player::components::Player;
use bevy_rapier2d::prelude::*;
use crate::{RENDER_SCALE, RENDER_SIZE};


pub fn spawn_enemy(
    mut commands: Commands,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if !spawn_timer.timer.finished() { return;}

    spawn_timer.timer = Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Once);

    let window_length = Vec2::new(RENDER_SIZE.x / 2.0, RENDER_SIZE.y / 2.0).length();

    let player_transform = player_query.get_single().unwrap();

    for _ in 0..5 {
        let mut rnd = rand::thread_rng();
        let spawn_position = Vec2::new(rnd.gen_range(-1.0..=1.0), rnd.gen_range(-1.0..=1.0)).normalize() * (window_length * RENDER_SCALE);


        let layout = TextureAtlasLayout::from_grid(Vec2::splat(32.0), 2, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let mut animation_config = AnimationConfig::new(0, 1, 5);

        animation_config.frame_timer = AnimationConfig::timer_from_fps(animation_config.fps);
        let mut sprite_bundle =  SpriteBundle {
            transform: Transform::from_xyz(spawn_position.x + player_transform.translation.x,
                                           spawn_position.y + player_transform.translation.y,
                                           0.0),
            texture: asset_server.load("sprites/mage.png"),
            ..default()
        };

        sprite_bundle.sprite.color = Color::rgb(1.0, 0.0,0.0);

        commands.spawn(
            (
                   sprite_bundle,
                    TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: animation_config.first_sprite_index
                    },
                    animation_config,
                    Enemy {},
                    Character {
                        direction: Vec2::default(),
                        size: 64,
                        speed: 20.0
                    },
                    RigidBody::Dynamic,
                    LockedAxes::ROTATION_LOCKED_Z,
                    Damping { linear_damping: 100.0, angular_damping: 1.0 },
                    Collider::capsule(Vec2::new(0.0, -5.0), Vec2::new(0.0, 5.0), 8.0)
                )
        );
    }
}

pub fn despawn_enemy(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>
) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&mut Character, &Transform), With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (mut character, transform) in enemy_query.iter_mut() {
            let dir = (player_transform.translation - transform.translation).normalize();

            character.direction.x = dir.x;
            character.direction.y = dir.y;
        }
    }
}

pub fn update_enemy_timer (
    mut enemy_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>
) {
    enemy_timer.timer.tick(time.delta());
}