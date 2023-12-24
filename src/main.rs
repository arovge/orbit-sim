mod commands;
mod components;
mod resources;
mod state;
mod systems;

use bevy::prelude::*;
use commands::SpawnAsteroidCommand;
use resources::*;
use state::*;
use systems::{
    asteroid_drag::AsteroidDragPlugin, input::InputPlugin, physics::PhysicsPlugin,
    planet::PlanetPlugin, ui::UiPlugin,
};

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.add(SpawnAsteroidCommand);
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .init_resource::<AsteroidDragResource>()
        .add_systems(Startup, setup)
        .add_plugins((
            AsteroidDragPlugin,
            InputPlugin,
            PhysicsPlugin,
            PlanetPlugin,
            UiPlugin,
        ))
        .run();
}
