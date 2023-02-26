use crate::components::{GameState, CelestialBody, CelestialType};
use bevy::prelude::*;

pub fn handle_escape_key_pressed(
    keys: Res<Input<KeyCode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if keys.any_just_pressed([KeyCode::Q, KeyCode::Escape]) {
        app_exit_events.send(bevy::app::AppExit);
    }
}

pub fn handle_reset_key_pressed(
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut query: Query<&mut CelestialBody>,
) {
    if keys.just_pressed(KeyCode::R) {
        query
            .iter_mut()
            .filter(|q| *q.celestial_type() == CelestialType::Asteroid)
            .for_each(|mut q| q.reset());
        state.set(GameState::Following).unwrap();
    }
}
