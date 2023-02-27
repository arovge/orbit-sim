use crate::components::{GameState, Asteroid};
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
    mut query: Query<&mut Asteroid>,
) {
    if keys.just_pressed(KeyCode::R) {
        query
            .iter_mut()
            .for_each(|mut q| q.reset());
        _ = state.set(GameState::FollowingCursor);
    }
}
