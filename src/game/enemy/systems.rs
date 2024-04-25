use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use crate::game::character::components::Character;
use crate::game::enemy::components::Enemy;
use crate::game::enemy::ENEMY_SPAWN_TIME;
use crate::game::enemy::resources::EnemySpawnTimer;
use crate::game::player::components::Player;
use bevy_rapier2d::prelude::*;


pub fn spawn_enemy(
    mut commands: Commands,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    if !spawn_timer.timer.finished() { return;}

    spawn_timer.timer = Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Once);

    let window = window_query.get_single().unwrap();

    let window_length = Vec2::new(window.width() / 2.0, window.height() / 2.0).length();

    let player_transform = player_query.get_single().unwrap();

    for i in 0..5 {
        let mut rnd = rand::thread_rng();
        let spawn_position = Vec2::new(rnd.gen_range(-1.0..=1.0), rnd.gen_range(-1.0..=1.0)).normalize() * window_length;
        commands.spawn(
            (
                    SpriteBundle {
                        transform: Transform::from_xyz(spawn_position.x + player_transform.translation.x,
                                                       spawn_position.y + player_transform.translation.y,
                                                       0.0),
                        texture: asset_server.load("sprites/temp.png"),
                        ..default()
                    },
                    Enemy {},
                    Character {
                        direction: Vec2::default(),
                        size: 64,
                        speed: 20.0
                    },
                    RigidBody::Dynamic,
                    LockedAxes::ROTATION_LOCKED_Z,
                    Damping { linear_damping: 100.0, angular_damping: 1.0 },
                    Collider::capsule(Vec2::new(0.0, -16.0), Vec2::new(0.0, 16.0), 20.0)
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