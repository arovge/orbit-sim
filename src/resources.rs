use bevy::prelude::{Resource, Vec2};

#[derive(Resource, Debug, Default)]
pub struct MouseDragResource {
    // TODO: Maybe this can be stored somewhere better. I originally wanted this to be part of
    // game state, but `derive(States) only supports fieldless enums`.
    // This doesn't really work with the type system and it should
    start_drag_location: Option<Vec2>,
}

impl MouseDragResource {
    pub fn start_drag_location(&self) -> Option<Vec2> {
        self.start_drag_location
    }

    pub fn set_start_drag_location(&mut self, location: Vec2) {
        self.start_drag_location = Some(location);
    }
}
