use crate::state::GameState;
use crate::{components::*, resources::MouseDragResource};
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

// TODO: Using this makes me feel like something is wrong somewhere
// Try refactoring match so this isn't needed
const MOUSE_SCALE: f32 = 1e10;

pub fn handle_asteroid_drag_start(
    mut state: ResMut<NextState<GameState>>,
    mut mouse_drag_resource: ResMut<MouseDragResource>,
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let cursor_position = windows.get_single().unwrap().cursor_position().unwrap();
        mouse_drag_resource.set_start_drag_location(cursor_position);
        state.set(GameState::CursorDragStarted);
    }
}

pub fn handle_asteroid_drag_end(
    mut state: ResMut<NextState<GameState>>,
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
        state.set(GameState::InOrbit);
    }
}

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

pub fn handle_edit_planets(
    windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    mut planets_query: Query<(Entity, &Transform, &Radius), (With<Planet>, Without<Asteroid>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = windows.get_single().unwrap();
        let half_width = window.width() / 2.;
        let half_height = window.height() / 2.;
        let cursor_position = window.cursor_position().unwrap();

        let position = Vec3::new(
            cursor_position.x - half_width,
            half_height - cursor_position.y,
            0.,
        );

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(50.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(position),
                ..default()
            },
            Planet,
            Mass::new(5.972e25),
            Radius::new(50.),
        ));
    }
    if buttons.just_pressed(MouseButton::Right) {
        let window = windows.get_single().unwrap();
        let half_width = window.width() / 2.;
        let half_height = window.height() / 2.;
        let cursor_position = window.cursor_position().unwrap();
        let delete_position = Vec3::new(
            cursor_position.x - half_width,
            half_height - cursor_position.y,
            0.,
        );

        for (planet_entity, planet_transform, radius) in planets_query.iter_mut() {
            let distance = planet_transform.translation.distance(delete_position);
            if distance < radius.radius() {
                commands.entity(planet_entity).despawn();
            }
        }
    }
}
