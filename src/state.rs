use bevy::prelude::Component;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Component)]
pub enum GameState {
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
            Self::EditPlanets => "Insert planet".to_owned(),
        }
    }
}
