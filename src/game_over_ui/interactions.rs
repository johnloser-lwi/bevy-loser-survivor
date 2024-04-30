use bevy::prelude::*;
use crate::game_over_ui::components::MainMenuButton;
use crate::states::AppState;
use crate::styles::{BUTTON_HOVER_COLOR, BUTTON_PRESSED_COLOR};

pub fn handle_main_menu_button (
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (With<MainMenuButton>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<AppState>>
) {
        for (interaction, mut background_color) in interaction_query.iter_mut() {
            match *interaction {
                Interaction::Hovered => {
                    background_color.0 = BUTTON_HOVER_COLOR;
                },
                Interaction::None => {
                    background_color.0 = Color::rgb(0.0, 0.0, 0.0);
                },
                Interaction::Pressed => {
                    background_color.0 = BUTTON_PRESSED_COLOR;

                    next_state.set(AppState::MainMenu);
                }
            }
        }
}