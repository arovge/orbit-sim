mod commands;
mod components;
mod plugins;
mod state;

use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use commands::SpawnAsteroidCommand;
use components::Velocity;
use plugins::{
    WithAsteroid, asteroid_drag::AsteroidDragPlugin, orbit::OrbitPlugin,
    planet_editor::PlanetEditorPlugin, ui::UiPlugin,
};
use state::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            AsteroidDragPlugin,
            DefaultPlugins,
            OrbitPlugin,
            PlanetEditorPlugin,
            UiPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                quit.run_if(
                    input_just_pressed(KeyCode::KeyQ).or(input_just_pressed(KeyCode::Escape)),
                ),
                reset_state.run_if(input_just_pressed(KeyCode::KeyR)),
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.queue(SpawnAsteroidCommand);
}

fn quit(mut writer: MessageWriter<AppExit>) {
    writer.write(AppExit::Success);
}

fn reset_state(
    mut asteroid: Single<&mut Velocity, WithAsteroid>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    asteroid.reset();
    next_state.set(GameState::FollowingCursor);
}
