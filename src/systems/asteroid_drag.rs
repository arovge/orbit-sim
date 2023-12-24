use crate::components::*;
use crate::resources::AsteroidDragResource;
use crate::state::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// TODO: Using this makes me feel like something is wrong somewhere
// Try refactoring match so this isn't needed
const MOUSE_SCALE: f32 = 1e10;

pub struct AsteroidDragPlugin;

impl Plugin for AsteroidDragPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_cursor_moved.run_if(
                in_state(GameState::FollowingCursor).or_else(in_state(GameState::EditPlanets)),
            ),
        )
        .add_systems(
            Update,
            handle_cursor_drag_start.run_if(in_state(GameState::FollowingCursor)),
        )
        .add_systems(
            Update,
            (handle_cursor_drag_end,).run_if(in_state(GameState::CursorDragStarted)),
        );
    }
}

fn handle_cursor_moved(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, (With<Asteroid>, Without<Planet>)>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let half_width = window.width() / 2.;
    let half_height = window.height() / 2.;

    let mut asteroid_transform = query.single_mut();
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    asteroid_transform.translation.x = cursor_position.x - half_width;
    asteroid_transform.translation.y = half_height - cursor_position.y;
}

fn handle_cursor_drag_start(
    mut next_state: ResMut<NextState<GameState>>,
    mut asteroid_drag_resource: ResMut<AsteroidDragResource>,
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let cursor_position = windows.single().cursor_position().unwrap();
        asteroid_drag_resource.drag_start_position = Some(cursor_position);
        next_state.set(GameState::CursorDragStarted);
    }
}

fn handle_cursor_drag_end(
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<&mut Velocity, (With<Asteroid>, Without<Planet>)>,
    mut asteroid_drag_resource: ResMut<AsteroidDragResource>,
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if buttons.just_released(MouseButton::Left) {
        // The mouse drag may end outside of the window
        let Some(end_cursor_position) = windows.single().cursor_position() else {
            asteroid_drag_resource.reset();
            next_state.set(GameState::FollowingCursor);
            return;
        };
        let start_cursor_position = asteroid_drag_resource.drag_start_position.unwrap();
        let x = end_cursor_position.x - start_cursor_position.x;
        let y = start_cursor_position.y - end_cursor_position.y;

        let mut asteroid_velocity = query.single_mut();
        asteroid_velocity.set(x * MOUSE_SCALE, y * MOUSE_SCALE);
        next_state.set(GameState::InOrbit);
    }
}
