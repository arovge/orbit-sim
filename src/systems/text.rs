use crate::components::{Asteroid, CoordinatesText, Planet, StateText};
use crate::state::GameState;
use bevy::prelude::*;

pub fn update_state_text(
    state: Res<State<GameState>>,
    mut query: Query<&mut Text, With<StateText>>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = state.current().description();
    }
}

pub fn update_coordinates_text(
    mut text_query: Query<&mut Text, With<CoordinatesText>>,
    asteroid_query: Query<&Transform, (With<Asteroid>, Without<Planet>)>,
) {
    let asteroid_translation = asteroid_query.single().translation;
    for mut text in text_query.iter_mut() {
        text.sections[0].value =
            format!("{0}, {1}", asteroid_translation.x, asteroid_translation.y);
    }
}
