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
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut asteroid_query: Query<&mut Transform, (With<Asteroid>, Without<Planet>)>,
) {
    let Some(cursor_position) = window_query.single().cursor_position() else {
        return;
    };

    let (camera, camera_transform) = camera_query.single();
    let position = camera
        .viewport_to_world_2d(camera_transform, cursor_position)
        .unwrap();

    let mut asteroid_transform = asteroid_query.single_mut();
    asteroid_transform.translation.x = position.x;
    asteroid_transform.translation.y = position.y;
}

fn handle_cursor_drag_start(
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    mut asteroid_drag_resource: ResMut<AsteroidDragResource>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let cursor_position = window_query.single().cursor_position().unwrap();

        let (camera, camera_transform) = camera_query.single();
        let start_position = camera
            .viewport_to_world_2d(camera_transform, cursor_position)
            .unwrap();

        asteroid_drag_resource.drag_start_position = Some(start_position);
        next_state.set(GameState::CursorDragStarted);
    }
}

fn handle_cursor_drag_end(
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    mut asteroid_query: Query<&mut Velocity, (With<Asteroid>, Without<Planet>)>,
    mut asteroid_drag_resource: ResMut<AsteroidDragResource>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if buttons.just_released(MouseButton::Left) {
        let Some(end_cursor_position) = window_query.single().cursor_position() else {
            asteroid_drag_resource.reset();
            next_state.set(GameState::FollowingCursor);
            return;
        };
        let (camera, camera_transform) = camera_query.single();
        let end_position = camera
            .viewport_to_world_2d(camera_transform, end_cursor_position)
            .unwrap();

        let start_position = asteroid_drag_resource.drag_start_position.unwrap();
        let x = end_position.x - start_position.x;
        let y = end_position.y - start_position.y;

        let mut asteroid_velocity = asteroid_query.single_mut();
        asteroid_velocity.0 = Vec2::new(x * MOUSE_SCALE, y * MOUSE_SCALE);
        next_state.set(GameState::InOrbit);
    }
}
