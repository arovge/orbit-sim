mod commands;
mod components;
mod resources;
mod state;
mod systems;

use bevy::prelude::*;
use resources::*;
use state::*;
use systems::{
    asteroid::AsteroidPlugin, cursor::CursorPlugin, input::InputPlugin, planet::PlanetPlugin,
    ui::UiPlugin,
};

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .init_resource::<CursorDragResource>()
        .add_systems(Startup, setup)
        .add_plugins((
            AsteroidPlugin,
            CursorPlugin,
            InputPlugin,
            PlanetPlugin,
            UiPlugin,
        ))
        .run();
}
