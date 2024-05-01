use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game::upgrade_menu::components::UpgradeOption;
use crate::game::weapons::fire_ball::FireBallData;
use crate::game::weapons::force_field::ForceFieldData;
use crate::game::weapons::whip::WhipData;
use crate::styles::get_button_bundle;

#[derive(Component)]
pub struct LevelUpUI {}

pub fn spawn_level_up_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    whip_data: Res<WhipData>,
    force_field_data: Res<ForceFieldData>,
    fire_ball_data: Res<FireBallData>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    let level_up_parent = (
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
        },
        LevelUpUI{}
        );

    let level_up_popup =

        NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(30.0),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::DARK_GRAY.into(),
            ..default()
        }
        ;

    let title_text =
        TextBundle::from_section(
            "Pick An Upgrade",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 0.054 * window.height(),
                color: Color::rgb(1.0, 1.0, 1.0),
            },
        );

    let option1 = random_option(&whip_data, &fire_ball_data, &force_field_data);
    let mut option2 = random_option(&whip_data, &fire_ball_data, &force_field_data);
    let mut option3 = random_option(&whip_data, &fire_ball_data, &force_field_data);

    while option1 == option2 {
        option2 = random_option(&whip_data, &fire_ball_data, &force_field_data);
    }

    while option3 == option1 || option3 == option2 {
        option3 = random_option(&whip_data, &fire_ball_data, &force_field_data);
    }

    commands.spawn(level_up_parent).with_children(|commands| {
        commands.spawn(level_up_popup).with_children(|commands| {
            commands.spawn(title_text);
            spawn_button(commands, &asset_server, &option1, window.height());
            spawn_button(commands, &asset_server, &option2, window.height());
            spawn_button(commands, &asset_server, &option3, window.height());
        });
    });
}



fn random_option(whip: &WhipData, fire_ball: &FireBallData, force_field: &ForceFieldData) -> UpgradeOption {
    loop {
        let random = UpgradeOption::pick_random_option();
        if random == UpgradeOption::GetWhip && whip.data.level != 0 {
            continue;
        }

        if random == UpgradeOption::GetFireBall && fire_ball.data.level != 0 {
            continue;
        }

        if random == UpgradeOption::GetForceField && force_field.data.level != 0 {
            continue;
        }

        if random == UpgradeOption::FireBallProjectile && fire_ball.data.level == 0 {
            continue;
        }

        if random == UpgradeOption::FireBallDamage && fire_ball.data.level == 0 {
            continue;
        }

        if random == UpgradeOption::FireBallSpeed && fire_ball.data.level == 0 {
            continue;
        }

        if random == UpgradeOption::FireBallCoolDown && fire_ball.data.level == 0 {
            continue;
        }

        if random == UpgradeOption::ForceFieldProjectile && force_field.data.level == 0 {
            continue;
        }

        if random == UpgradeOption::ForceFieldDamage && force_field.data.level == 0 {
            continue;
        }

        if random == UpgradeOption::ForceFieldLifeTime && force_field.data.level == 0 {
            continue;
        }

        if random == UpgradeOption::ForceFieldCoolDown && force_field.data.level == 0 {
            continue;
        }

        if random == UpgradeOption::WhipCount && whip.data.level == 0 {
            continue;
        }

        if random == UpgradeOption::WhipDamage && whip.data.level == 0 {
            continue;
        }

        if random == UpgradeOption::WhipCoolDown && whip.data.level == 0 {
            continue;
        }

        return random;
    }
}

fn spawn_button(
    commands: &mut ChildBuilder,
    asset_server: &AssetServer,
    upgrade_option: &UpgradeOption,
    window_height: f32
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let button = (
        get_button_bundle(Val::Percent(80.0), Val::Percent(30.0)),
        upgrade_option.clone(),
    );

    let text = upgrade_option.get_name();

    let button_text = TextBundle {
        text: Text::from_section(
            text,
            TextStyle {
                font: font.clone(),
                font_size: 0.027 * window_height,
                color: Color::WHITE,
            },
        ),
        ..default()
    };

    commands.spawn(button).with_children(|commands| {
        commands.spawn(button_text);
    });

}

pub fn despawn_level_up_ui (
    mut commands: Commands,
    level_up_ui: Query<Entity, With<LevelUpUI>>
) {
    if let Ok(ui) = level_up_ui.get_single() {
        commands.entity(ui).despawn_recursive();
    }

}