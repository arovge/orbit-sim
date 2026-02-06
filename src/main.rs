mod commands;
mod components;
mod plugins;
mod state;

use bevy::prelude::*;
use commands::SpawnAsteroidCommand;
use plugins::{
    asteroid_drag::AsteroidDragPlugin, input::InputPlugin, physics::PhysicsPlugin,
    planet_editor::PlanetEditorPlugin, ui::UiPlugin,
};
use state::*;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.queue(SpawnAsteroidCommand);
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            AsteroidDragPlugin,
            DefaultPlugins,
            InputPlugin,
            PhysicsPlugin,
            PlanetEditorPlugin,
            UiPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .run();
}
