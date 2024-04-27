use crate::game::resources::*;
use bevy::prelude::*;
use crate::game::coin::components::Coin;
use crate::game::events::{OnEnemyDie, OnPickupCoin};
use crate::game::player::components::Player;


pub fn spawn_coin(
    mut commands: Commands,
    mut enemy_die_event_reader: EventReader<OnEnemyDie>,
    textures: Res<Textures>,
) {

    for evt in enemy_die_event_reader.read() {
        commands.spawn(
            (
                SpriteBundle {
                    texture: textures.coin.clone(),
                    transform: Transform::from_xyz(evt.position.x, evt.position.y, -2.0),
                    ..default()
                },
                Coin {
                    xp: 1
                },
            )

        );
    }
}

pub fn update_coins(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    coin_query: Query<(Entity, &Transform, &Coin)>,
    mut pickup_event_writer: EventWriter<OnPickupCoin>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (entity, transform, coin) in coin_query.iter() {
            if Vec2::distance(player_transform.translation.truncate(), transform.translation.truncate()) < 20.0 {
                pickup_event_writer.send(OnPickupCoin { xp: coin.xp });

                commands.entity(entity).despawn_recursive();
            }
        }
    }
}