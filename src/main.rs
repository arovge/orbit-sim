mod bundles;
mod components;
mod systems;

use bevy::prelude::*;
use bundles::*;
use components::*;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(CelestialBundle::planet(
        50.,
        5.972e25,
        &mut meshes,
        &mut materials,
    ));
    commands.spawn(CelestialBundle::asteroid(
        10.,
        5.972e25,
        &mut meshes,
        &mut materials,
    ));
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Following)
        .add_startup_system(setup)
        .add_system(systems::handle_escape_key_pressed)
        .add_system_set(
            SystemSet::on_update(GameState::Following)
                .with_system(systems::handle_mouse_motion)
                .with_system(systems::handle_mouse_input),
        )
        .add_system_set(
            SystemSet::on_update(GameState::FreeFall)
                .with_system(systems::handle_freefall)
                .with_system(systems::handle_reset_key_pressed),
        )
        .run();
}
