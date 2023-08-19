use crate::components::*;
use bevy::window::PrimaryWindow;
use bevy::prelude::*;

pub fn handle_cursor_moved(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, (With<Asteroid>, Without<Planet>)>,
) {
    let Ok(window) = windows.get_single() else { return };
    let half_width = window.width() / 2.;
    let half_height = window.height() / 2.;

    let mut asteroid_transform = query.single_mut();
    let Some(cursor_position) = window.cursor_position() else { return };
    asteroid_transform.translation.x = cursor_position.x - half_width;
    asteroid_transform.translation.y = half_height - cursor_position.y;
}
