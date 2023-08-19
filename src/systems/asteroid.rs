use crate::state::GameState;
use crate::{components::*, resources::MouseDragResource};
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

// TODO: Using this makes me feel like something is wrong somewhere
// Try refactoring match so this isn't needed
const MOUSE_SCALE: f32 = 1e10;

pub fn handle_asteroid_drag_start(
    mut next_state: ResMut<NextState<GameState>>,
    mut mouse_drag_resource: ResMut<MouseDragResource>,
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let cursor_position = windows.get_single().unwrap().cursor_position().unwrap();
        mouse_drag_resource.set_start_drag_location(cursor_position);
        next_state.set(GameState::CursorDragStarted);
    }
}

pub fn handle_asteroid_drag_end(
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<&mut Velocity, (With<Asteroid>, Without<Planet>)>,
    game_resource: Res<MouseDragResource>,
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if buttons.just_released(MouseButton::Left) {
        let end_cursor_position = windows.get_single().unwrap().cursor_position().unwrap();
        let start_cursor_position = game_resource.start_drag_location().unwrap();
        let x = end_cursor_position.x - start_cursor_position.x;
        let y = start_cursor_position.y - end_cursor_position.y;

        let mut asteroid_velocity = query.single_mut();
        asteroid_velocity.set(x * MOUSE_SCALE, y * MOUSE_SCALE);
        next_state.set(GameState::InOrbit);
    }
}
