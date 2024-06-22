use bevy::prelude::{Component, States};
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Component, States, Default)]
pub enum GameState {
    #[default]
    FollowingCursor,
    AsteroidDragStarted,
    InOrbit,
    EditPlanets,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Self::FollowingCursor => "Following cursor",
            Self::AsteroidDragStarted => "Dragging",
            Self::InOrbit => "In orbit",
            Self::EditPlanets => "Edit planets",
        };
        write!(f, "{str}")
    }
}
