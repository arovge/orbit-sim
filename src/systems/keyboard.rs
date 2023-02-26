use bevy::prelude::*;

use crate::components::GameState;

pub fn handle_escape_key_pressed(
    keys: Res<Input<KeyCode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        app_exit_events.send(bevy::app::AppExit);
    }
}

pub fn handle_reset_key_pressed(
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
) {
    if keys.just_pressed(KeyCode::R) {
        state.set(GameState::Following).unwrap();
    }
}
