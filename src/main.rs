mod components;
mod resources;
mod state;
mod systems;

use crate::components::{StateText, *};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use resources::*;
use state::*;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    let font = asset_server.load("fonts/clacon2.ttf");
    let text_style = TextStyle {
        font,
        font_size: 18.0,
        color: Color::WHITE,
    };

    commands.spawn((
        TextBundle::from_section(GameState::FollowingCursor.description(), text_style.clone())
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(15.),
                left: Val::Px(15.),
                ..default()
            })
            .with_text_alignment(TextAlignment::Left),
        StateText,
    ));
    commands.spawn((
        TextBundle::from_section("0, 0", text_style)
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(15.),
                right: Val::Px(15.),
                ..default()
            })
            .with_text_alignment(TextAlignment::Right),
        CoordinatesText,
    ));
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

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .init_resource::<MouseDragResource>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                systems::keyboard::check_for_exit_key_press,
                systems::keyboard::check_for_insert_mode_toggle,
                systems::text::update_state_text,
                systems::text::update_coordinates_text,
            ),
        )
        .add_systems(
            Update,
            (
                systems::mouse::handle_cursor_moved,
                systems::mouse::handle_asteroid_drag_start,
            )
                .run_if(in_state(GameState::FollowingCursor)),
        )
        .add_systems(
            Update,
            (
                systems::keyboard::check_for_reset_key_press,
                systems::mouse::handle_asteroid_drag_end,
            )
                .run_if(in_state(GameState::CursorDragStarted)),
        )
        .add_systems(
            Update,
            (
                systems::orbit::handle_orbit,
                systems::keyboard::check_for_reset_key_press,
            )
                .run_if(in_state(GameState::InOrbit)),
        )
        .add_systems(
            Update,
            (
                systems::keyboard::check_for_insert_mode_toggle,
                systems::mouse::handle_cursor_moved,
                systems::mouse::handle_edit_planets,
            )
                .run_if(in_state(GameState::EditPlanets)),
        )
        .run();
}
