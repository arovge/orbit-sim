use crate::components::*;
use bevy::prelude::*;

pub fn handle_mouse_input(mut state: ResMut<State<GameState>>, buttons: Res<Input<MouseButton>>) {
    if buttons.just_pressed(MouseButton::Left) {
        state.set(GameState::FreeFall).unwrap();
    }
}

pub fn handle_mouse_motion(windows: Res<Windows>, mut query: Query<(&mut Transform, &CelestialType)>) {
    let window = windows.get_primary().unwrap();
    let half_width = window.width() / 2.;
    let half_height = window.height() / 2.;

    query
        .iter_mut()
        .filter(|q| *q.1 == CelestialType::Asteroid)
        .for_each(|mut q| {
            let Some(cursor_position) = window.cursor_position() else { return };
            q.0.translation.x = cursor_position.x - half_width;
            q.0.translation.y = cursor_position.y - half_height;
        });
}
