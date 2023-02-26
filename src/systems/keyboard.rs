use crate::components::{CelestialBody, CelestialType, GameState};
use bevy::prelude::*;

pub fn check_for_exit_key_press(
    keys: Res<Input<KeyCode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if keys.any_just_pressed([KeyCode::Q, KeyCode::Escape]) {
        app_exit_events.send(bevy::app::AppExit);
    }
}

pub fn check_for_reset_key_press(
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
    mut query: Query<&mut CelestialBody>,
) {
    if keys.just_pressed(KeyCode::R) {
        query
            .iter_mut()
            .filter(|q| *q.celestial_type() == CelestialType::Asteroid)
            .for_each(|mut q| q.reset());
        state.set(GameState::FollowingCursor).unwrap();
    }
}
