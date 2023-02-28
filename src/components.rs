use bevy::prelude::*;

pub mod components {
    pub use crate::components::*;
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Component)]
pub enum GameState {
    FollowingCursor,
    CursorDragStarted,
    InOrbit,
}

impl GameState {
    pub fn description(&self) -> String {
        match self {
            Self::FollowingCursor => "Following cursor".to_owned(),
            Self::CursorDragStarted => "Dragging".to_owned(),
            Self::InOrbit => "In orbit".to_owned(),
        }
    }
}

#[derive(Component)]
pub struct StateText;

#[derive(Component)]
pub struct Asteroid;

#[derive(Component)]
pub struct Radius(f32);

impl Radius {
    pub fn new(radius: f32) -> Self {
        Self(radius)
    }

    pub fn radius(&self) -> f32 {
        self.0
    }
}

#[derive(Component)]
pub struct Velocity(Vec3);

impl Default for Velocity {
    fn default() -> Self {
        Self(Vec3::default())
    }
}

impl Velocity {
    pub fn accelerate(&mut self, x_acceleration: f32, y_acceleration: f32) {
        self.0.x += x_acceleration;
        self.0.y += y_acceleration;
    }

    pub fn set(&mut self, x_velocity: f32, y_velocity: f32) {
        self.0.x = x_velocity;
        self.0.y = y_velocity;
    }

    pub fn velocity(&self) -> Vec3 {
        self.0
    }

    pub fn reset(&mut self) {
        self.0 = Vec3::default();
    }
}

#[derive(Component)]
pub struct Planet;

#[derive(Component)]
pub struct Mass(f32);

impl Mass {
    pub fn new(mass: f32) -> Self {
        Self(mass)
    }

    pub fn mass(&self) -> f32 {
        self.0
    }
}
