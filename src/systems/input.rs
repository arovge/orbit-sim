use crate::components::{Asteroid, Planet, Velocity};
use crate::state::GameState;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_for_exit_key_press,
                check_for_reset_key_press,
                check_for_insert_mode_toggle,
            ),
        );
    }
}

fn check_for_exit_key_press(
    keys: Res<Input<KeyCode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if keys.any_just_pressed([KeyCode::Q, KeyCode::Escape]) {
        app_exit_events.send(bevy::app::AppExit);
    }
}

fn check_for_reset_key_press(
    keys: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut query: Query<&mut Velocity, (With<Asteroid>, Without<Planet>)>,
) {
    if keys.just_pressed(KeyCode::R) {
        let mut asteroid_velocity = query.single_mut();
        asteroid_velocity.reset();
        next_state.set(GameState::FollowingCursor);
    }
}

fn check_for_insert_mode_toggle(
    keys: Res<Input<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::I) {
        match *state.get() {
            GameState::EditPlanets => next_state.set(GameState::FollowingCursor),
            _ => next_state.set(GameState::EditPlanets),
        };
    }
}
