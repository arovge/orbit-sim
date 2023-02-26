use bevy::prelude::{Resource, Vec2};

#[derive(Resource, Debug)]
pub struct MouseDragResource {
    // TODO: Maybe this can be stored somewhere better. I originally wanted this to be part of
    // game state, but f32/f64 doesn't conform to the Hash trait.
    // Maybe could serialize it to a string but that also feels wrong
    // This doesn't really work with the type system and it should
    start_drag_location: Option<Vec2>,
}

impl Default for MouseDragResource {
    fn default() -> Self {
        Self {
            start_drag_location: None,
        }
    }
}

impl MouseDragResource {
    pub fn start_drag_location(&self) -> Option<Vec2> {
        self.start_drag_location
    }

    pub fn set_start_drag_location(&mut self, location: Vec2) {
        self.start_drag_location = Some(location);
    }
}
