use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game_over_ui::components::{GameOverUI, MainMenuButton};
use crate::game::gameplay::resources::GameplayData;
use crate::styles::{get_button_bundle, get_column_layout};

pub fn spawn_game_over_ui (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gameplay_data: Res<GameplayData>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    let ui_parent = (
        get_column_layout(),
        GameOverUI
    );

    let game_over_text =
        TextBundle::from_section("Game Over",
                                 TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 0.054 * window.height(),
            color: Color::rgb(1.0, 1.0, 1.0),
        }
    );

    let result_text =
        TextBundle::from_section(get_gameplay_text(&gameplay_data),
                                 TextStyle {
                                     font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                     font_size: 0.027 * window.height(),
                                     color: Color::rgb(1.0, 1.0, 1.0),
                                 }
        );

    let button = (
        get_button_bundle(Val::Percent(15.6), Val::Percent(5.4)),
        MainMenuButton
    );

    let button_text = TextBundle {
        text: Text::from_section(
            "Main Menu",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 0.027 * window.height(),
                color: Color::WHITE,
            },
        ),
        ..default()
    };



    commands.spawn(ui_parent).with_children(|parent| {
        parent.spawn(game_over_text);
        parent.spawn(result_text);
        parent.spawn(button).with_children(|button| {
            button.spawn(button_text);
        });
    });
}


fn get_gameplay_text(gameplay_data: &GameplayData) -> String {

    let hours = (gameplay_data.game_time / 3600.0) as u32;
    let minutes = ((gameplay_data.game_time - (hours as f32 * 3600.0)) / 60.0) as u32;
    let seconds = (gameplay_data.game_time - (hours as f32 * 3600.0) - (minutes as f32 * 60.0)) as u32;
    let time = if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    };

    format!("Survival Time: {}\nLevel: {}\nXP: {}\nHead Count: {}", time, gameplay_data.level, gameplay_data.xp, gameplay_data.head_count)

}

pub fn despawn_game_over_ui (mut commands: Commands, query: Query<Entity, With<GameOverUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
