use bevy::prelude::{Resource, Vec2};

#[derive(Resource, Debug)]
pub struct GameResource {
    // TODO: Maybe this can be stored somewhere better. I originally wanted this to be part of
    // game state, but f32/f64 doesn't conform to the Hash trait.
    // Maybe could serialize it to a string but that also feels wrong
    // This doesn't really work with the type system and it should
    initial_mouse_drag_location: Option<Vec2>,
}

impl Default for GameResource {
    fn default() -> Self {
        Self {
            initial_mouse_drag_location: None,
        }
    }
}

impl GameResource {
    pub fn initial_mouse_drag_location(&self) -> Option<Vec2> {
        self.initial_mouse_drag_location
    }

    pub fn set_initial_mouse_drag_location(&mut self, location: Vec2) {
        self.initial_mouse_drag_location = Some(location);
    }
}
