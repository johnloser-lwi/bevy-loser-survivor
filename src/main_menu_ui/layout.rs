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
    let mut column = get_column_layout();

    column.style.justify_content = JustifyContent::FlexEnd;

    let ui_parent = (
        column,
        MainMenuUI
    );

    let bg = ImageBundle {
        image: asset_server.load("sprites/main_menu.png").into(),
        style: Style {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        ..default()
    };

    let mut button_bundle = get_button_bundle(Val::Percent(12.0), Val::Percent(4.0));
    button_bundle.style.margin.bottom = Val::Percent(20.0);
    button_bundle.style.margin.left = Val::Percent(45.0);

    let button = (
        button_bundle,
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
            parent.spawn(bg);
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