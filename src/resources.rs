use bevy::prelude::{Resource, Vec2};

#[derive(Resource, Debug, Default)]
pub struct AsteroidDragResource {
    // TODO: Maybe this can be stored somewhere better. I originally wanted this to be part of
    // game state, but `derive(States) only supports fieldless enums`.
    // This doesn't really work with the type system and it should
    pub drag_start_position: Option<Vec2>,
}

impl AsteroidDragResource {
    pub fn reset(&mut self) {
        self.drag_start_position = None;
    }
}
