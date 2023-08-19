use crate::state::GameState;
use crate::{components::*, resources::MouseDragResource};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;

// TODO: Using this makes me feel like something is wrong somewhere
// Try refactoring match so this isn't needed
const MOUSE_SCALE: f32 = 1e10;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_asteroid)
            .add_systems(
                Update,
                handle_asteroid_drag_start.run_if(in_state(GameState::FollowingCursor)),
            )
            .add_systems(
                Update,
                (handle_asteroid_drag_end,).run_if(in_state(GameState::CursorDragStarted)),
            );
    }
}

fn setup_asteroid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Asteroid,
        Radius::new(10.),
        Velocity::default(),
    ));
}

fn handle_asteroid_drag_start(
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

fn handle_asteroid_drag_end(
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
