use bevy::prelude::*;

use crate::components::{GameState, StateText};

pub fn update_state_text(
    state: Res<State<GameState>>,
    mut query: Query<&mut Text, With<StateText>>,
) {
    query
        .iter_mut()
        .for_each(|mut text| {
            text.sections[0].value = state.current().description();
        });
}
