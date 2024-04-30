use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use crate::audio::events::RequestSpatialAudioEvent;
use crate::game::character::components::{DamageFlash, Health};
use crate::game::enemy::components::Enemy;
use crate::game::player::components::Player;
use crate::game::resources::{Sounds, Textures};
use crate::game::weapons::WeaponData;

#[derive(Resource)]
pub struct ForceFieldData {
    pub data: WeaponData,
    pub life_time: f32,
    pub hit_time: f32
}

impl Default for ForceFieldData {
    fn default() -> Self {
        let data = ForceFieldData {
            data: WeaponData {
                level: 0,
                damage: 1.0,
                cooldown: 3.0,
                count: 0,
                timer: Vec::new()
            },
            life_time: 4.0,
            hit_time: 1.0
        };

        data
    }
}

#[derive(Component)]
pub struct ForceField {
    life_time: f32,
    hit_timer: Timer
}

pub fn insert_force_field_data(
    mut commands: Commands
) {
    commands.insert_resource(ForceFieldData::default());
}

pub fn remove_force_field_data (
    mut commands: Commands
) {
    commands.remove_resource::<ForceFieldData>();
}


pub fn spawn_force_field(
    mut commands: Commands,
    player_query: Query<&GlobalTransform, With<Player>>,
    textures: Res<Textures>,
    mut force_field_data: ResMut<ForceFieldData>,
    time: Res<Time>
) {

    for i in 0..force_field_data.data.timer.len() {
        let timer = force_field_data.data.timer.get_mut(i).unwrap();

        timer.tick(time.delta());

        if !timer.just_finished() { continue; }

        force_field_data.data.reset_timer(i);

        if force_field_data.data.level == 0 { continue; }


        if let Ok(player_transform) = player_query.get_single() {

            let mut rng = rand::thread_rng();

            let cast_radius = 100.0 * rng.gen_range(0.3..=1.0);

            let dir = Vec2::new( rng.gen_range(-1.0..=1.0),  rng.gen_range(-1.0..=1.0)).normalize();
            let pos = dir * cast_radius + player_transform.translation().truncate();
            let mut sprite_bundle = SpriteBundle {
                texture: textures.force_field.clone(),
                transform: Transform::from_xyz(pos.x, pos.y, 1.0),
                ..default()
            };
            sprite_bundle.sprite.color = Color::rgba(1.0, 1.0, 1.0, 0.3);
            commands.spawn(
                (
                    sprite_bundle,
                    ForceField {
                        life_time: force_field_data.life_time,
                        hit_timer: Timer::from_seconds(force_field_data.hit_time, TimerMode::Repeating)
                    },
                    Sensor,
                    Collider::ball(24.0)
                )
            );
        }

    }
}

pub fn update_force_field (
    mut commands: Commands,
    mut force_field_query: Query<(Entity, &mut ForceField, &Collider, &GlobalTransform)>,
    force_field_data: Res<ForceFieldData>,
    mut enemy_query: Query<(&mut Health, &mut DamageFlash, &mut Sprite), With<Enemy>>,
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    sounds: Res<Sounds>,
    mut spatial_audio_request: EventWriter<RequestSpatialAudioEvent>
){
    for (entity, mut force_field, collider, global_transform) in force_field_query.iter_mut() {
        force_field.life_time -= time.delta_seconds();
        if force_field.life_time <= 0.0 {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        force_field.hit_timer.tick(time.delta());
        if !force_field.hit_timer.just_finished() { continue; }

        // collision
        rapier_context.intersections_with_shape(
            global_transform.translation().truncate(),
            0.0,
            collider,
            QueryFilter::new(),
            |entity| {
                if let Ok((mut health, mut damage_flash, mut sprite)) = enemy_query.get_mut(entity) {
                    health.take_damage(force_field_data.data.damage);
                    damage_flash.flash(&mut sprite);

                    spatial_audio_request.send(RequestSpatialAudioEvent {
                        position: global_transform.translation().truncate(),
                        sound: sounds.enemy_damage.clone()
                    });

                }
                true
            }
        );

    }
}


pub fn despawn_force_field (
    mut commands: Commands,
    entity_query: Query<Entity, With<ForceField>>
) {
    for entity in entity_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}