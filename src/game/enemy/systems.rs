use bevy::prelude::*;
use rand::prelude::*;
use crate::game::animation::components::AnimationConfig;
use crate::game::character::components::{Character, DamageFlash, Health};
use crate::game::enemy::components::Enemy;
use crate::game::enemy::resources::{EnemyConfigurations, EnemySpawnTimer, EnemyConfig};
use crate::game::player::components::Player;
use crate::resources::Textures;
use bevy_rapier2d::prelude::*;
use crate::{RENDER_SCALE, RENDER_SIZE};
use crate::game::character::resources::CharacterTextureAtlasLayout;
use crate::game::events::{OnEnemyDie, OnLevelUp};
use crate::game::gameplay::resources::GameplayData;
use crate::game::health_bar::components::HealthBar;


pub fn setup_enemy_config(
    mut commands: Commands,
    textures: Res<Textures>
){
    commands.insert_resource(EnemyConfigurations {
        configs: vec![
            EnemyConfig {
                speed: 20.0,
                health: 5.0,
                damage:20.0,
                texture: textures.zombie.clone(),
                collider: Collider::capsule(Vec2::new(0.0, -4.0), Vec2::new(0.0, 4.0), 6.0)
            }
        ],
        ..default()
    });
}

pub fn spawn_enemy(
    mut commands: Commands,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    player_query: Query<&Transform, With<Player>>,
    atlas_layout: Res<CharacterTextureAtlasLayout>,
    enemy_configurations: Res<EnemyConfigurations>
) {
    if !spawn_timer.timer.finished() { return;}

    spawn_timer.timer = Timer::from_seconds(enemy_configurations.spawn_time, TimerMode::Once);

    let window_length = Vec2::new(RENDER_SIZE.x / 2.0, RENDER_SIZE.y / 2.0).length();

    let player_transform = player_query.get_single().unwrap();

    for _ in 0..=enemy_configurations.spawn_count {
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
            texture: active_config.texture.clone(),
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
                        speed: active_config.speed
                    },
                    Health {
                        health: active_config.health,
                        max_health: active_config.health,
                        regeneration: 0.0
                    },
                    HealthBar::default(),
                    DamageFlash{
                        timer: Timer::default(),
                        color: Color::RED
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

pub fn cleanup_enemy(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (entity, transform) in enemy_query.iter() {
            if Vec2::distance(player_transform.translation.truncate(), transform.translation.truncate()) > RENDER_SIZE.x / 2.0 {
                commands.entity(entity).despawn_recursive();
            }
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
    mut player_query: Query<(&mut Health, &mut DamageFlash, &mut Sprite), With<Player>>,
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

                if let Ok((mut health, mut damage_flash, mut sprite)) = player_query.get_mut(entity) {
                    health.take_damage(enemy.damage * time.delta_seconds());
                    damage_flash.flash(&mut sprite);
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

pub fn enemy_level_up (
    mut enemy_configurations: ResMut<EnemyConfigurations>,
    mut level_up_event: EventReader<OnLevelUp>,
    gameplay_data: Res<GameplayData>
) {


    for evt in level_up_event.read() {
        if gameplay_data.level % 5 != 0 {continue;}
        println!("Level up enemy");
        for mut config in enemy_configurations.configs.iter_mut() {
            config.speed *= 1.2;
            config.damage *= 1.2;
            config.health *= 1.1;
        }
        enemy_configurations.spawn_time *= 0.9;
        enemy_configurations.spawn_count += 1;
    }
}