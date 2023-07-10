use crate::components::{Asteroid, Planet, Velocity};
use crate::state::GameState;
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
    mut state: ResMut<NextState<GameState>>,
    mut query: Query<&mut Velocity, (With<Asteroid>, Without<Planet>)>,
) {
    if keys.just_pressed(KeyCode::R) {
        let mut asteroid_velocity = query.single_mut();
        asteroid_velocity.reset();
        state.set(GameState::FollowingCursor);
    }
}

pub fn check_for_insert_mode_toggle(
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::I) {
        _ = match state.0 {
            Some(GameState::EditPlanets) => state.set(GameState::FollowingCursor),
            _ => state.set(GameState::EditPlanets),
        };
    }
}
