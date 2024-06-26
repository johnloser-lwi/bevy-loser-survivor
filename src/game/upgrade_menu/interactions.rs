use bevy::prelude::*;
use crate::game::character::components::{Character, Health};
use crate::game::player::components::Player;
use crate::game::upgrade_menu::components::UpgradeOption;
use crate::game::weapons::fire_ball::FireBallData;
use crate::game::weapons::force_field::ForceFieldData;
use crate::game::weapons::whip::WhipData;
use crate::states::GameState;
use crate::styles::{BUTTON_HOVER_COLOR, BUTTON_PRESSED_COLOR};

pub fn level_up_button_system (
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &UpgradeOption), (With<Button>, Changed<Interaction>)>,
    mut whip_data: ResMut<WhipData>,
    mut fire_ball_data: ResMut<FireBallData>,
    mut force_field_data: ResMut<ForceFieldData>,
    mut player_query: Query<(&mut Health, &mut Character), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>
) {
    for (interaction, mut background_color, upgrade_option) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                background_color.0 = BUTTON_PRESSED_COLOR;


                match upgrade_option {
                    UpgradeOption::GetWhip => {
                        whip_data.data.level += 1;
                        whip_data.data.count = 1;
                    },
                    UpgradeOption::GetFireBall => {
                        fire_ball_data.data.level += 1;
                        fire_ball_data.data.add_timer();
                    },
                    UpgradeOption::GetForceField => {
                        force_field_data.data.level += 1;
                        force_field_data.data.add_timer();
                    }
                    UpgradeOption::FireBallProjectile => {
                        fire_ball_data.data.level += 1;
                        fire_ball_data.data.add_timer();
                    }
                    UpgradeOption::FireBallDamage => {
                        fire_ball_data.data.level += 1;
                        fire_ball_data.data.damage *= 1.1;
                    }
                    UpgradeOption::FireBallSpeed => {
                        fire_ball_data.data.level += 1;
                        fire_ball_data.speed *= 1.1;
                    }
                    UpgradeOption::FireBallCoolDown => {
                        fire_ball_data.data.level += 1;
                        fire_ball_data.data.cooldown *= 0.9;
                    }
                    UpgradeOption::ForceFieldProjectile => {
                        force_field_data.data.level += 1;
                        force_field_data.data.add_timer();
                    }
                    UpgradeOption::ForceFieldDamage => {
                        force_field_data.data.level += 1;
                        force_field_data.data.damage *= 1.1;
                    }
                    UpgradeOption::ForceFieldLifeTime => {
                        force_field_data.data.level += 1;
                        force_field_data.life_time *= 1.1;
                    }
                    UpgradeOption::ForceFieldCoolDown => {
                        force_field_data.data.level += 1;
                        force_field_data.data.cooldown *= 0.9;
                    }
                    UpgradeOption::WhipCount => {
                        whip_data.data.level += 1;
                        whip_data.data.add_timer();
                    }
                    UpgradeOption::WhipDamage => {
                        whip_data.data.level += 1;
                        whip_data.data.damage *= 1.1;
                    }
                    UpgradeOption::WhipCoolDown => {
                        whip_data.data.level += 1;
                        whip_data.data.cooldown *= 0.9;
                    }
                    UpgradeOption::HealthUp => {
                        for (mut health, _) in player_query.iter_mut() {
                            health.max_health *= 1.1;
                        }
                    }
                    UpgradeOption::SpeedUp => {
                        for (_, mut character) in player_query.iter_mut() {
                            character.speed *= 1.1;
                        }
                    }
                    UpgradeOption::RegenerationUp => {
                        for (mut health, _) in player_query.iter_mut() {
                            health.regeneration += 0.1;
                        }
                    }
                }




                next_state.set(GameState::Running);
            },
            Interaction::Hovered => {
                background_color.0 = BUTTON_HOVER_COLOR;
            },
            Interaction::None => {
                background_color.0 = Color::BLACK;
            }
        }
    }
}