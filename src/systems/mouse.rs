use crate::components::*;
use crate::state::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_cursor_moved.run_if(
                in_state(GameState::FollowingCursor).or_else(in_state(GameState::EditPlanets)),
            ),
        );
    }
}

fn handle_cursor_moved(
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
