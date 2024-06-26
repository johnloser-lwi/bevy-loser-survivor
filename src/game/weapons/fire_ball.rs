use bevy::prelude::*;
use bevy::utils::FloatOrd;
use bevy_rapier2d::prelude::*;
use crate::audio::events::RequestSpatialAudioEvent;
use crate::game::character::components::{DamageFlash, Health};
use crate::game::enemy::components::Enemy;
use crate::game::player::components::Player;
use crate::game::resources::{Sounds, Textures};
use crate::game::weapons::WeaponData;


#[derive(Resource)]
pub struct FireBallData {
    pub data: WeaponData,
    pub life_time: f32,
    pub speed: f32
}

impl Default for FireBallData {
    fn default() -> Self {
        let data = FireBallData {
            data: WeaponData {
                level: 0,
                damage: 5.0,
                cooldown: 1.5,
                count: 0,
                timer: Vec::new()
            },
            life_time: 2.0,
            speed: 100.0
        };
        data
    }
}

#[derive(Component)]
pub struct FireBall {
    life_time: f32,
    direction: Vec2,
    speed: f32
}

pub fn insert_fire_ball_data(
    mut commands: Commands
) {
    commands.insert_resource(FireBallData::default());
}

pub fn remove_fire_ball_data (
    mut commands: Commands
) {
    commands.remove_resource::<FireBallData>();
}

pub fn spawn_fire_ball(
    mut commands: Commands,
    player_query: Query<&GlobalTransform, With<Player>>,
    textures: Res<Textures>,
    mut fire_ball_data: ResMut<FireBallData>,
    time: Res<Time>,
    enemy_query: Query<&Transform, With<Enemy>>,
    sounds: Res<Sounds>,
    mut spatial_event: EventWriter<RequestSpatialAudioEvent>
) {
    if let Ok(player_transform) = player_query.get_single() {

        for i in 0..fire_ball_data.data.timer.len() {
            let timer = fire_ball_data.data.timer.get_mut(i).unwrap();

            timer.tick(time.delta());

            if !timer.just_finished() { continue; }
            fire_ball_data.data.reset_timer(i);

            if let Some(closest_enemy) = enemy_query.iter().min_by_key(|enemy_transform| {
                FloatOrd(Vec2::length(
                    player_transform.translation().truncate() - enemy_transform.translation.truncate(),
                ))
            }) {

                let dir = (closest_enemy.translation.truncate() - player_transform.translation().truncate()).normalize();

                let sprite_bundle = SpriteBundle {
                    texture: textures.fire_ball.clone(),
                    transform: Transform::from_xyz(player_transform.translation().x, player_transform.translation().y, 1.0),
                    ..default()
                };
                commands.spawn(
                    (
                        sprite_bundle,
                        FireBall {
                            life_time: fire_ball_data.life_time,
                            direction: dir,
                            speed: fire_ball_data.speed
                        },
                        Sensor,
                        Collider::ball(2.0)
                    )
                );

                spatial_event.send(RequestSpatialAudioEvent {
                    position: player_transform.translation().truncate(),
                    sound: sounds.fire_ball.clone()
                });

            }


        }

    }






}

pub fn update_fire_ball (
    mut commands: Commands,
    mut fire_ball_query: Query<(Entity, &mut FireBall, &Collider, &mut Transform)>,
    fire_ball_data: Res<FireBallData>,
    mut enemy_query: Query<(&mut Health, &mut DamageFlash, &mut Sprite), With<Enemy>>,
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    sound: Res<Sounds>,
    mut spatial_event: EventWriter<RequestSpatialAudioEvent>
){
    for (entity, mut fire_ball, collider, mut transform) in fire_ball_query.iter_mut() {
        fire_ball.life_time -= time.delta_seconds();
        if fire_ball.life_time <= 0.0 {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        let velocity = fire_ball.direction * (fire_ball.speed * time.delta_seconds());
        let translation = Vec3::new(velocity.x, velocity.y, 0.0);

        transform.translation += translation;


        // collision
        rapier_context.intersections_with_shape(
            transform.translation.truncate(),
            0.0,
            collider,
            QueryFilter::new(),
            |entity| {
                if let Ok((mut health, mut damage_flash, mut sprite)) = enemy_query.get_mut(entity) {
                    health.take_damage(fire_ball_data.data.damage);
                    damage_flash.flash(&mut sprite);
                    fire_ball.life_time = 0.0;

                    spatial_event.send(RequestSpatialAudioEvent {
                        position: transform.translation.truncate(),
                        sound: sound.enemy_damage.clone()
                    });
                }
                true
            }
        );

    }
}


pub fn despawn_fire_ball (
    mut commands: Commands,
    entity_query: Query<Entity, With<FireBall>>
) {
    for entity in entity_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
