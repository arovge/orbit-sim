use crate::components::*;
use crate::state::GameState;
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            handle_edit_planets.run_if(in_state(GameState::EditPlanets)),
        );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(-350., 150., 0.)),
            ..default()
        },
        Planet,
        Mass::new(5.972e25),
        Radius::new(50.),
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Planet,
        Mass::new(5.972e25),
        Radius::new(50.),
    ));
}

fn handle_edit_planets(
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
                break;
            }
        }
    }
}
