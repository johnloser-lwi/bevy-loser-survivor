use bevy::prelude::*;
use crate::game::game_ui::components::{LevelText, TimeText, XpBar};
use crate::game::gameplay::resources::GameplayData;

pub fn update_xp_ui(
    gameplay_data: Res<GameplayData>,
    mut xp_bar_query: Query<&mut Style, With<XpBar>>,
    mut level_text_query: Query<&mut Text, (With<LevelText>, Without<TimeText>)>,
    mut time_text_query: Query<&mut Text, (With<TimeText>, Without<LevelText>)>
) {
    for mut style in xp_bar_query.iter_mut() {

        let last_level_xp = gameplay_data.xp_to_next_level - gameplay_data.get_xp_offset_to_next_level();
        let xp_diff = gameplay_data.get_xp_offset_to_next_level();
        let xp_diff_progress = gameplay_data.xp - last_level_xp;


        style.width = Val::Percent(xp_diff_progress as f32 / xp_diff as f32 * 100.0);
    }

    for mut text in level_text_query.iter_mut() {
        text.sections[0].value = format!("Lv.{}", gameplay_data.level);
    }

    for mut text in time_text_query.iter_mut() {
        //text.sections[0].value = format!("{:.2}", gameplay_data.game_time);
        // format time text to HH:MM:SS
        let hours = (gameplay_data.game_time / 3600.0) as u32;
        let minutes = ((gameplay_data.game_time - (hours as f32 * 3600.0)) / 60.0) as u32;
        let seconds = (gameplay_data.game_time - (hours as f32 * 3600.0) - (minutes as f32 * 60.0)) as u32;
        if hours > 0 {
            text.sections[0].value = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
        } else {
            text.sections[0].value = format!("{:02}:{:02}", minutes, seconds);
        }
    }
}