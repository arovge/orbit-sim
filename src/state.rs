use bevy::prelude::{Component, States};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Component, States, Default)]
pub enum GameState {
    #[default]
    FollowingCursor,
    CursorDragStarted,
    InOrbit,
    EditPlanets,
}

impl GameState {
    pub fn description(&self) -> String {
        match self {
            Self::FollowingCursor => "Following cursor".to_owned(),
            Self::CursorDragStarted => "Dragging".to_owned(),
            Self::InOrbit => "In orbit".to_owned(),
            Self::EditPlanets => "Edit planets".to_owned(),
        }
    }
}
