use crate::{components::*, resources::MouseDragResource};
use bevy::prelude::*;

// TODO: Using this makes me feel like something is wrong somewhere
// Try refactoring match so this isn't needed
const MOUSE_SCALE: f32 = 1e10;

pub fn handle_asteroid_drag_start(
    mut state: ResMut<State<GameState>>,
    mut mouse_drag_resource: ResMut<MouseDragResource>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let cursor_position = windows.get_primary().unwrap().cursor_position().unwrap();
        mouse_drag_resource.set_start_drag_location(cursor_position);
        _ = state.set(GameState::CursorDragStarted);
    }
}

pub fn handle_asteroid_drag_end(
    mut state: ResMut<State<GameState>>,
    mut query: Query<&mut Velocity, (With<Asteroid>, Without<Planet>)>,
    game_resource: Res<MouseDragResource>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    if buttons.just_released(MouseButton::Left) {
        let end_cursor_position = windows.get_primary().unwrap().cursor_position().unwrap();
        let start_cursor_position = game_resource.start_drag_location().unwrap();
        let x = end_cursor_position.x - start_cursor_position.x;
        let y = end_cursor_position.y - start_cursor_position.y;

        let mut asteroid_velocity = query.single_mut();
        asteroid_velocity.set(x * MOUSE_SCALE, y * MOUSE_SCALE);
        _ = state.set(GameState::InOrbit);
    }
}

pub fn handle_cursor_moved(
    windows: Res<Windows>,
    mut query: Query<&mut Transform, (With<Asteroid>, Without<Planet>)>,
) {
    let window = windows.get_primary().unwrap();
    let half_width = window.width() / 2.;
    let half_height = window.height() / 2.;

    let mut asteroid_transform = query.single_mut();
    let Some(cursor_position) = window.cursor_position() else { return };
    asteroid_transform.translation.x = cursor_position.x - half_width;
    asteroid_transform.translation.y = cursor_position.y - half_height;
}
