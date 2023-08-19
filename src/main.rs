mod components;
mod resources;
mod state;
mod systems;

use bevy::prelude::*;
use resources::*;
use state::*;
use systems::{
    asteroid::AsteroidPlugin, keyboard::KeyboardPlugin, mouse::MousePlugin, orbit::OrbitPlugin,
    planet::PlanetPlugin, text::TextPlugin,
};

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .init_resource::<MouseDragResource>()
        .add_systems(Startup, setup)
        .add_plugins((
            AsteroidPlugin,
            KeyboardPlugin,
            MousePlugin,
            OrbitPlugin,
            PlanetPlugin,
            TextPlugin,
        ))
        .run();
}
