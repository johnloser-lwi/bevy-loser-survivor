use bevy::prelude::*;
use crate::game::background::components::Background;
use crate::RENDER_SIZE;


pub fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let bg = asset_server.load("sprites/background.png");

    commands.spawn(
        (
        SpriteBundle {
            transform: Transform::from_xyz(-RENDER_SIZE.x / 2.0, -RENDER_SIZE.y / 2.0, -10.0),
            texture: bg.clone(),
            ..Default::default()
        },
            Background{}
        )
    );

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(RENDER_SIZE.x / 2.0, -RENDER_SIZE.y / 2.0, -10.0),
                texture: bg.clone(),
                ..Default::default()
            },
            Background{}
        )
    );

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(-RENDER_SIZE.x / 2.0, RENDER_SIZE.y / 2.0, -10.0),
                texture: bg.clone(),
                ..Default::default()
            },
            Background{}
        )
    );

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(RENDER_SIZE.x / 2.0, RENDER_SIZE.y / 2.0, -10.0),
                texture: bg.clone(),
                ..Default::default()
            },
            Background{}
        )
    );
}

pub fn despawn_background(
    mut commands: Commands,
    background_query: Query<Entity, With<Background>>
)
{
    for entity in background_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}