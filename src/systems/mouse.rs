use crate::components::*;
use bevy::prelude::*;

pub fn handle_mouse_input(mut state: ResMut<State<GameState>>, buttons: Res<Input<MouseButton>>) {
    if buttons.just_pressed(MouseButton::Left) {
        // Left button was pressed
        println!("left pressed");
        state.set(GameState::FreeFall).unwrap();
    }
    if buttons.just_released(MouseButton::Left) {
        // Left Button was released
        println!("left released");
    }
    if buttons.pressed(MouseButton::Right) {
        // Right Button is being held down

        println!("right pressed");
    }
    // we can check multiple at once with `.any_*`
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        // Either the left or the right button was just pressed
        println!("left or right pressed");
    }
}

pub fn handle_mouse_motion(state: Res<State<GameState>>, windows: Res<Windows>, mut query: Query<(&mut Transform, &CelestialType)>) {
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
