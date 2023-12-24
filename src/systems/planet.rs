use crate::commands::SpawnPlanetCommand;
use crate::components::*;
use crate::state::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            handle_edit_planets.run_if(in_state(GameState::EditPlanets)),
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

fn handle_edit_planets(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    buttons: Res<Input<MouseButton>>,
    mut planets_query: Query<(Entity, &Transform, &Radius), (With<Planet>, Without<Asteroid>)>,
    mut commands: Commands,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = window_query.single();
        let cursor_position = window.cursor_position().unwrap();

        let (camera, camera_transform) = camera_query.single();
        let position = camera
            .viewport_to_world_2d(camera_transform, cursor_position)
            .unwrap()
            .extend(0.);

        commands.add(SpawnPlanetCommand { position });
    }

    if buttons.just_pressed(MouseButton::Right) {
        let window = window_query.single();
        let cursor_position = window.cursor_position().unwrap();

        let (camera, camera_transform) = camera_query.single();
        let position = camera
            .viewport_to_world_2d(camera_transform, cursor_position)
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
}
