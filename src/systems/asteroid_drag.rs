use super::{world_position_2d, WithAsteroid};
use crate::components::*;
use crate::state::GameState;
use bevy::input::common_conditions::{input_just_pressed, input_just_released};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// TODO: Using this makes me feel like something is wrong somewhere
// Try refactoring match so this isn't needed
const MOUSE_SCALE: f32 = 1e10;

// TODO: Maybe this can be stored somewhere better. I originally wanted this to be part of
// game state, but `derive(States) only supports fieldless enums`.
// This doesn't really work with the type system and it should
// Could do something with Local<T>. But then the start & end drag systems
// would need to be combined into one. And there wouldn't be an easy way to update
// The UI in the bottom right corner to say what's happening/get the asteroid
// to stop following the planet when drag starts
#[derive(Resource, Debug, Default)]
struct AsteroidDragStartPosition(pub Option<Vec2>);

impl AsteroidDragStartPosition {
    fn reset(&mut self) {
        self.0 = None;
    }
}

pub struct AsteroidDragPlugin;

impl Plugin for AsteroidDragPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsteroidDragStartPosition::default())
            .add_systems(
                Update,
                handle_cursor_moved.run_if(
                    in_state(GameState::FollowingCursor).or(in_state(GameState::EditPlanets)),
                ),
            )
            .add_systems(
                Update,
                handle_asteroid_drag_start.run_if(
                    in_state(GameState::FollowingCursor).and(input_just_pressed(MouseButton::Left)),
                ),
            )
            .add_systems(
                Update,
                handle_asteroid_drag_end.run_if(
                    in_state(GameState::AsteroidDragStarted)
                        .and(input_just_released(MouseButton::Left)),
                ),
            );
    }
}

fn handle_cursor_moved(
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut asteroid_query: Query<&mut Transform, WithAsteroid>,
) {
    let Some(position) = world_position_2d(&window_query, &camera_query) else {
        return;
    };

    let mut asteroid_transform = asteroid_query.single_mut();
    asteroid_transform.translation.x = position.x;
    asteroid_transform.translation.y = position.y;
}

fn handle_asteroid_drag_start(
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut asteroid_drag_start_position: ResMut<AsteroidDragStartPosition>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(start_position) = world_position_2d(&window_query, &camera_query) else {
        return;
    };

    asteroid_drag_start_position.0 = Some(start_position);
    next_state.set(GameState::AsteroidDragStarted);
}

fn handle_asteroid_drag_end(
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut asteroid_query: Query<&mut Velocity, WithAsteroid>,
    mut asteroid_drag_start_position: ResMut<AsteroidDragStartPosition>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(end_position) = world_position_2d(&window_query, &camera_query) else {
        asteroid_drag_start_position.reset();
        next_state.set(GameState::FollowingCursor);
        return;
    };

    let start_position = asteroid_drag_start_position.0.unwrap();
    let x = end_position.x - start_position.x;
    let y = end_position.y - start_position.y;

    let mut asteroid_velocity = asteroid_query.single_mut();
    asteroid_velocity.0 = Vec2::new(x * MOUSE_SCALE, y * MOUSE_SCALE);
    next_state.set(GameState::InOrbit);
}
