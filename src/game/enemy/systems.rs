use bevy::prelude::*;
use rand::prelude::*;
use crate::game::animation::components::AnimationConfig;
use crate::game::character::components::{ Character, Health};
use crate::game::enemy::components::Enemy;
use crate::game::enemy::ENEMY_SPAWN_TIME;
use crate::game::enemy::resources::{EnemyConfigurations, EnemySpawnTimer};
use crate::game::player::components::Player;
use bevy_rapier2d::prelude::*;
use crate::{RENDER_SCALE, RENDER_SIZE};
use crate::game::character::resources::CharacterTextureAtlasLayout;
use crate::game::events::OnEnemyDie;
use crate::game::gamplay::resources::GameplayData;


pub fn spawn_enemy(
    mut commands: Commands,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    atlas_layout: Res<CharacterTextureAtlasLayout>,
    enemy_configurations: Res<EnemyConfigurations>
) {
    if !spawn_timer.timer.finished() { return;}

    spawn_timer.timer = Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Once);

    let window_length = Vec2::new(RENDER_SIZE.x / 2.0, RENDER_SIZE.y / 2.0).length();

    let player_transform = player_query.get_single().unwrap();

    for _ in 0..5 {
        let mut rnd = rand::thread_rng();
        let spawn_position = Vec2::new(rnd.gen_range(-1.0..=1.0), rnd.gen_range(-1.0..=1.0)).normalize() * (window_length * RENDER_SCALE);

        let config_count = enemy_configurations.configs.iter().count();

        let active_config_index = rnd.gen_range(0..config_count);

        let active_config = enemy_configurations.configs.get(active_config_index).unwrap();

        let animation_config = AnimationConfig::new(0, 1, 5);

        let sprite_bundle =  SpriteBundle {
            transform: Transform::from_xyz(spawn_position.x + player_transform.translation.x,
                                           spawn_position.y + player_transform.translation.y,
                                           0.0),
            texture: asset_server.load(active_config.texture.clone()),
            ..default()
        };

        commands.spawn(
            (
                   sprite_bundle,
                    TextureAtlas {
                        layout: atlas_layout.handle.clone(),
                        index: animation_config.first_sprite_index
                    },
                    animation_config,
                    Enemy {
                        damage: active_config.damage
                    },
                    Character {
                        direction: Vec2::default(),
                        size: active_config.size,
                        speed: active_config.speed
                    },
                    Health {
                        health: active_config.health
                    },
                    RigidBody::Dynamic,
                    LockedAxes::ROTATION_LOCKED_Z,
                    Damping { linear_damping: 100.0, angular_damping: 1.0 },
                    active_config.collider.clone()
                )
        );
    }
}

pub fn despawn_enemy(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>
) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn_recursive();
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

pub fn damage_player(
    enemy_query: Query<(&Collider, &GlobalTransform, &Enemy)>,
    mut player_query: Query<&mut Health, With<Player>>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for (collider, transform, enemy) in enemy_query.iter() {
        rapier_context.intersections_with_shape(
            transform.translation().truncate(),
            0.0,
            collider,
            QueryFilter::new(),
            |entity| {

                if let Ok(mut health) = player_query.get_mut(entity) {
                    health.take_damage(enemy.damage * time.delta_seconds());
                }

                true
            },

        );
    }
}

pub fn check_enemy_health(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Health, &GlobalTransform), With<Enemy>>,
    mut enemy_die_event_writer: EventWriter<OnEnemyDie>
) {
    for (entity, health, transform) in enemy_query.iter() {
        if health.is_dead() {
            commands.entity(entity).despawn_recursive();
            enemy_die_event_writer.send(OnEnemyDie { position : transform.translation().truncate() });
        }
    }
}