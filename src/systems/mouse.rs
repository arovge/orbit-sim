use crate::{components::*, resources::GameResource};
use bevy::prelude::*;

// TODO: Using this makes me feel like something is wrong somewhere
// Try refactoring match so this isn't needed
const MOUSE_SCALE: f32 = 1e10;

pub fn handle_left_cursor_released(
    mut state: ResMut<State<GameState>>,
    mut query: Query<&mut CelestialBody>,
    game_resource: Res<GameResource>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    if buttons.just_released(MouseButton::Left) {
        let end_cursor_position = windows.get_primary().unwrap().cursor_position().unwrap();
        let start_cursor_position = game_resource.initial_mouse_drag_location().unwrap();
        let x = end_cursor_position.x - start_cursor_position.x;
        let y = end_cursor_position.y - start_cursor_position.y;

        query
            .iter_mut()
            .filter(|q| *q.celestial_type() == CelestialType::Asteroid)
            .for_each(|mut q| {
                q.set_velocity(x * MOUSE_SCALE, y * MOUSE_SCALE);
            });

        state.set(GameState::InOrbit).unwrap();
    }
}

pub fn handle_mouse_input(
    mut state: ResMut<State<GameState>>,
    mut game_resource: ResMut<GameResource>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let cursor_position = windows.get_primary().unwrap().cursor_position().unwrap();
        game_resource.set_initial_mouse_drag_location(cursor_position);

        state.set(GameState::CursorDragStarted).unwrap();
    }
}

pub fn handle_mouse_motion(
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &CelestialBody)>,
) {
    let window = windows.get_primary().unwrap();
    let half_width = window.width() / 2.;
    let half_height = window.height() / 2.;

    query
        .iter_mut()
        .filter(|q| *q.1.celestial_type() == CelestialType::Asteroid)
        .for_each(|mut q| {
            let Some(cursor_position) = window.cursor_position() else { return };
            q.0.translation.x = cursor_position.x - half_width;
            q.0.translation.y = cursor_position.y - half_height;
        });
}
