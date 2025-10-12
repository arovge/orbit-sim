use crate::components::Velocity;
use crate::state::GameState;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

use super::WithAsteroid;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_exit_key_press.run_if(
                    input_just_pressed(KeyCode::KeyQ).or(input_just_pressed(KeyCode::Escape)),
                ),
                handle_reset_key_press.run_if(input_just_pressed(KeyCode::KeyR)),
            ),
        );
    }
}

fn handle_exit_key_press(mut writer: MessageWriter<AppExit>) {
    writer.write(AppExit::Success);
}

fn handle_reset_key_press(
    mut asteroid_query: Single<&mut Velocity, WithAsteroid>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    asteroid_query.reset();
    next_state.set(GameState::FollowingCursor);
}
