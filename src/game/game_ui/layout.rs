use bevy::prelude::*;
use crate::game::game_ui::components::{GameUI, LevelText, TimeText, XpBar};

pub fn spawn_game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let parent_node = (
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(10.0),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        },
        GameUI
    );


    let xp_parent = (
        NodeBundle {
            style: Style {
                height: Val::Percent(20.0),
                width: Val::Percent(80.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                //flex_direction: FlexDirection::Row,
                //position_type: PositionType::Absolute,
                //margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::GRAY),
            ..default()
        }
    );

    let exp_node = (
        NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(50.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::DARK_GREEN),
            ..default()
        },
        XpBar
    );

    let time_text_parent =
        NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(5.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        };

    let time_text =
        (
            TextBundle::from_section(
                "00:00",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(1.0, 1.0, 1.0),
                }

            ),
            TimeText
        );

    let level_text_parent =
        NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        };

    let level_text =
        (
            TextBundle::from_section(
                "Lv.1",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 15.0,
                    color: Color::rgb(1.0, 1.0, 1.0),
                }

            ),
            LevelText
        );

    commands.spawn(parent_node).with_children(|commands| {
        commands.spawn(xp_parent).with_children(|parent| {
            parent.spawn(exp_node);
            parent.spawn(level_text_parent).with_children(|parent| {
                parent.spawn(level_text);
            });
        });

        commands.spawn(time_text_parent).with_children(|parent| {
            parent.spawn(time_text);
        });
    });
}

pub fn despawn_game_ui(
    mut commands: Commands,
    game_ui: Query<Entity, With<GameUI>>
) {
    if let Ok(entity) = game_ui.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}