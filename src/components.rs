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
pub struct Asteroid {
    radius: f64,
    velocity: Vec3,
    acceleration: Vec3,
}

impl Asteroid {
    pub fn new(radius: f64) -> Self {
        Self {
            radius,
            velocity: Vec3::default(),
            acceleration: Vec3::default(),
        }
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn velocity(&self) -> Vec3 {
        self.velocity
    }

    pub fn update_physics(&mut self, x_acceleration: f32, y_acceleration: f32) {
        self.acceleration.x = x_acceleration;
        self.acceleration.y = y_acceleration;
        self.velocity.x += x_acceleration;
        self.velocity.y += y_acceleration;
    }

    pub fn reset(&mut self) {
        self.velocity = Vec3::default();
        self.acceleration = Vec3::default();
    }
    
    pub fn set_velocity(&mut self, x: f32, y: f32) {
        self.velocity = Vec3::new(x, y, 0.);
    }
}

#[derive(Component)]
pub struct Planet {
    mass: f64,
    radius: f64,
}

impl Planet {
    pub fn new(mass: f64, radius: f64) -> Self {
        Self { mass, radius }
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
}
