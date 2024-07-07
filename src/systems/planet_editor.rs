use super::{world_position_2d, WithPlanet};
use crate::commands::SpawnPlanetCommand;
use crate::components::*;
use crate::state::GameState;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct PlanetEditorPlugin;

impl Plugin for PlanetEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                handle_insert_mode_toggle.run_if(input_just_pressed(KeyCode::KeyI)),
                handle_add_planet.run_if(
                    in_state(GameState::EditPlanets)
                        .and_then(input_just_pressed(MouseButton::Left)),
                ),
                handle_remove_planet.run_if(
                    in_state(GameState::EditPlanets)
                        .and_then(input_just_pressed(MouseButton::Right)),
                ),
            ),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.add(SpawnPlanetCommand {
        position: Vec3::new(-350., 150., 0.),
    });
    commands.add(SpawnPlanetCommand {
        position: Vec3::ZERO,
    });
}

fn handle_insert_mode_toggle(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match *state.get() {
        GameState::EditPlanets => next_state.set(GameState::FollowingCursor),
        _ => next_state.set(GameState::EditPlanets),
    };
}

fn handle_add_planet(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut commands: Commands,
) {
    let position = world_position_2d(&window_query, &camera_query)
        .unwrap()
        .extend(0.);

    commands.add(SpawnPlanetCommand { position });
}

fn handle_remove_planet(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut planets_query: Query<(Entity, &Transform, &Radius), WithPlanet>,
    mut commands: Commands,
) {
    let position = world_position_2d(&window_query, &camera_query)
        .unwrap()
        .extend(0.);

    for (planet_entity, planet_transform, radius) in planets_query.iter_mut() {
        let distance = planet_transform.translation.distance(position);
        if distance < radius.0 {
            commands.entity(planet_entity).despawn();
            break;
        }
    }
}
