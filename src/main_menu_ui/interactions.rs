use bevy::prelude::*;
use crate::main_menu_ui::components::StartButton;
use crate::states::AppState;
use crate::styles::{BUTTON_HOVER_COLOR, BUTTON_PRESSED_COLOR};

pub fn handle_start_button (
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (With<StartButton>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<AppState>>
) {
    for (interaction, mut background_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                background_color.0 = BUTTON_HOVER_COLOR;
            },
            Interaction::None => {
                background_color.0 = Color::BLACK;
            },
            Interaction::Pressed => {
                background_color.0 = BUTTON_PRESSED_COLOR;

                next_state.set(AppState::Loading);
            }
        }
    }
}