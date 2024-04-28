use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::game::resources::*;
use crate::game::animation::components::AnimationConfig;
use crate::game::character::components::Health;
use crate::game::enemy::components::Enemy;
use crate::game::player::components::Player;
use crate::game::weapons::WeaponData;

#[derive(Resource)]
pub struct WhipTextureAtlasLayout {
    pub handle: Handle<TextureAtlasLayout>
}

impl Default for WhipTextureAtlasLayout {
    fn default() -> Self {
        WhipTextureAtlasLayout {
            handle: Handle::default()
        }
    }
}

#[derive(Resource)]
pub struct WhipData {
    pub data: WeaponData
}

impl Default for WhipData {
    fn default() -> Self {
        let mut data = WhipData {
            data: WeaponData{
                level: 1,
                count: 1,
                damage: 5.0,
                cooldown: 3.0,
                timer: Timer::default()
            }
        };
        data.data.reset_timer();
        data
    }
}

#[derive(Component)]
pub struct Whip {
    offset: Vec2
}

pub fn insert_whip_data(
    mut commands: Commands
){
    commands.insert_resource(WhipData::default());
}

pub fn remove_whip_data(
    mut commands: Commands
) {
    commands.remove_resource::<WhipData>();
}

pub fn setup_whip_atlas(
    mut atlas: ResMut<WhipTextureAtlasLayout>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(Vec2::new(48.0, 16.0), 3, 1, None, None);
    atlas.handle = texture_atlas_layouts.add(layout);
}

pub fn spawn_whips(
    mut commands: Commands,
    mut whip_data: ResMut<WhipData>,
    time: Res<Time>,
    player_query: Query<(&Sprite, &GlobalTransform), With<Player>>,
    textures: Res<Textures>,
    atlas_layout: Res<WhipTextureAtlasLayout>
) {
    whip_data.data.timer.tick(time.delta());
    if !whip_data.data.timer.just_finished() {
        return;
    }

    whip_data.data.reset_timer();



    if whip_data.data.level == 0 {return;}


    if let Ok((sprite, transform)) = player_query.get_single() {
        for i in 0..whip_data.data.count {
            let animation_config = AnimationConfig::new(0, 2, 30);

            let mut flip = sprite.flip_x;
            if i == 1 {flip = !flip}
            let mut offset = Vec2::new(32.0, 0.0);
            if flip { offset.x = -offset.x;}

            let mut sprite_bundle =  SpriteBundle {
                transform: Transform::from_xyz(transform.translation().x + offset.x, transform.translation().y + offset.y, 1.0),
                texture: textures.whip.clone(),
                ..default()
            };
            sprite_bundle.sprite.flip_x = flip;

            sprite_bundle.sprite.color = Color::rgba(1.0, 1.0, 1.0, 0.5);


            commands.spawn(
                (
                        sprite_bundle,
                        TextureAtlas {
                            layout: atlas_layout.handle.clone(),
                            index: animation_config.first_sprite_index
                        },
                        animation_config,
                        Whip {
                            offset
                        },
                        Sensor,
                        Collider::cuboid(24.0, 2.0)
                    )
            );
        }
    }
}

pub fn update_whips(
    mut commands: Commands,
    time: Res<Time>,
    mut whip_query: Query<(Entity, &mut AnimationConfig, &mut TextureAtlas, &mut Transform, &GlobalTransform, &Whip, &Collider)>,
    player_query: Query<&GlobalTransform, With<Player>>,
    rapier_context: Res<RapierContext>,
    mut enemy_query: Query<&mut Health, With<Enemy>>,
    whip_data: Res<WhipData>
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (entity, mut config, mut atlas, mut transform, global_transform, whip, collider) in whip_query.iter_mut() {

            transform.translation.x = player_transform.translation().x + whip.offset.x;
            transform.translation.y = player_transform.translation().y + whip.offset.y;

            config.frame_timer.tick(time.delta());

            if config.frame_timer.just_finished() {
                if atlas.index == config.last_sprite_index {

                    // deal damage
                    rapier_context.intersections_with_shape(
                        global_transform.translation().truncate(),
                        0.0,
                        collider,
                        QueryFilter::new(),
                        |entity| {
                            if let Ok(mut health) = enemy_query.get_mut(entity) {
                                health.take_damage(whip_data.data.damage);
                            }
                            true
                        },
                    );
                    
                    commands.entity(entity).despawn_recursive();

                } else {
                    atlas.index += 1;
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }

}