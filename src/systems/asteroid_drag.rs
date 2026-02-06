use super::{WithAsteroid, world_position_2d};
use crate::components::*;
use crate::state::GameState;
use bevy::input::common_conditions::{input_just_pressed, input_just_released};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// TODO: Using this makes me feel like something is wrong somewhere
// Try refactoring match so this isn't needed
const MOUSE_SCALE: f32 = 1e10;

#[derive(Resource, Default)]
enum DragGesture {
    #[default]
    NotDragging,
    Dragging {
        start: Vec2,
    },
}

impl DragGesture {
    fn start_position(&self) -> Option<Vec2> {
        match self {
            DragGesture::Dragging { start } => Some(start.clone()),
            _ => None,
        }
    }

    fn reset(&mut self) {
        *self = DragGesture::default();
    }
}

pub struct AsteroidDragPlugin;

impl Plugin for AsteroidDragPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DragGesture::default())
            .add_systems(
                Update,
                handle_cursor_moved.run_if(
                    in_state(GameState::FollowingCursor).or(in_state(GameState::EditPlanets)),
                ),
            )
            .add_systems(
                Update,
                drag_start.run_if(
                    in_state(GameState::FollowingCursor).and(input_just_pressed(MouseButton::Left)),
                ),
            )
            .add_systems(
                Update,
                drag_end.run_if(
                    in_state(GameState::AsteroidDragStarted)
                        .and(input_just_released(MouseButton::Left)),
                ),
            );
    }
}

fn handle_cursor_moved(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut asteroid: Single<&mut Transform, WithAsteroid>,
) {
    let Some(position) = world_position_2d(&windows, &cameras) else {
        return;
    };

    asteroid.translation.x = position.x;
    asteroid.translation.y = position.y;
}

fn drag_start(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut drag_start: ResMut<DragGesture>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(start) = world_position_2d(&windows, &cameras) else {
        return;
    };

    *drag_start = DragGesture::Dragging { start };
    next_state.set(GameState::AsteroidDragStarted);
}

fn drag_end(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut asteroid: Single<&mut Velocity, WithAsteroid>,
    mut drag_gesture: ResMut<DragGesture>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(end_position) = world_position_2d(&windows, &cameras) else {
        drag_gesture.reset();
        next_state.set(GameState::FollowingCursor);
        return;
    };

    let Some(start_position) = drag_gesture.start_position() else {
        return;
    };

    asteroid.0 = (end_position - start_position) * MOUSE_SCALE;
    next_state.set(GameState::InOrbit);
}
