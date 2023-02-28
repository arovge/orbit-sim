use crate::components::{GameState, StateText};
use bevy::prelude::*;

pub fn update_state_text(
    state: Res<State<GameState>>,
    mut query: Query<&mut Text, With<StateText>>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = state.current().description();
    }
}
