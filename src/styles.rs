
use bevy::prelude::*;

pub const BUTTON_PRESSED_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
pub const BUTTON_HOVER_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);


pub fn get_button_bundle(width: Val, height: Val) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width,
            height,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            align_self: AlignSelf::Center,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        background_color: Color::CRIMSON.into(),
        ..default()
    }
}

pub fn get_column_layout() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        ..default()
    }
}