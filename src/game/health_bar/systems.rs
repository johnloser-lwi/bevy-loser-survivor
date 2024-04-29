use bevy::prelude::*;
use crate::game::character::components::Health;
use crate::game::health_bar::components::HealthBar;


pub fn setup_gizmo_config(
    mut config_store: ResMut<GizmoConfigStore>,
) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 5.0;
}
pub fn draw_health_bar (
    mut gizmos: Gizmos,
    health_query: Query<(&Transform, &Health, &HealthBar)>,
) {

    for (transform, health, health_bar) in health_query.iter() {

        if health.max_health == health.health || health.health <= 0.0 {
            continue;
        }

        let mid_pos = transform.translation.truncate() + health_bar.offset;
        let start_pos = Vec2::new(mid_pos.x - health_bar.size.x / 2.0, mid_pos.y);
        let end_pos = Vec2::new(start_pos.x + health.health / health.max_health * health_bar.size.x, start_pos.y);
        let full_pos = Vec2::new(mid_pos.x + health_bar.size.x / 2.0, mid_pos.y);

        gizmos.line_2d(start_pos, full_pos, Color::GRAY);
        gizmos.line_2d(start_pos, end_pos, Color::RED);

    }
}