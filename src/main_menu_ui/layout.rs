use bevy::prelude::*;
use crate::audio::events::RequestGlobalAudioEvent;
use crate::game::resources::Sounds;
use crate::main_menu_ui::components::{MainMenuUI, StartButton};
use crate::styles::{get_button_bundle, get_column_layout};

pub fn play_music(
    sounds: Res<Sounds>,
    mut request_global_audio_event: EventWriter<RequestGlobalAudioEvent>
) {
    request_global_audio_event.send(RequestGlobalAudioEvent {
        sound: sounds.music.clone(),
        is_loop: true
    });
}


pub fn spawn_main_menu_ui (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let ui_parent = (
        get_column_layout(),
        MainMenuUI
    );

    let title_text =
        TextBundle::from_section("Losers' World: Zombie Survivor",
                                 TextStyle {
                                     font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                     font_size: 60.0,
                                     color: Color::rgb(1.0, 1.0, 1.0),
                                 }
        );

    let button = (
        get_button_bundle(Val::Px(200.0), Val::Px(40.0)),
        StartButton
    );

    let button_text = TextBundle {
        text: Text::from_section(
            "Start",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
        ),
        ..default()
    };

    commands.spawn(ui_parent)
        .with_children(|parent| {
            parent.spawn(title_text);
            parent.spawn(button)
                .with_children(|parent| {
                    parent.spawn(button_text);
                });
        });
}

pub fn despawn_main_menu_ui (
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUI>>
){
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}