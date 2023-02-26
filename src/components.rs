use bevy::prelude::*;

pub mod components {
    pub use crate::components::*;
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Component)]
pub enum GameState {
    Following,
    FreeFall,
}

#[derive(Component)]
pub struct CelestialBody {
    radius: f32,
    mass: f64,
    celestial_type: CelestialType,

    // TODO: Is there a way for position to be owned by this component?
    // It seems like transform is owned by bundle.sprite on CelestialBundle
    velocity: Vec3,
    acceleration: Vec3,
}

impl CelestialBody {
    pub fn new(radius: f32, mass: f64, celestial_type: CelestialType) -> Self {
        Self {
            radius,
            mass,
            celestial_type,
            velocity: Vec3::default(),
            acceleration: Vec3::default(),
        }
    }

    pub fn reset(&mut self) {
        self.velocity = Vec3::default();
        self.acceleration = Vec3::default();
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn celestial_type(&self) -> &CelestialType {
        &self.celestial_type
    }

    pub fn velocity(&self) -> &Vec3 {
        &self.velocity
    }

    pub fn update_physics(&mut self, x_acceleration: f32, y_acceleration: f32) {
        self.acceleration.x = x_acceleration;
        self.acceleration.y = y_acceleration;
        self.velocity.x += x_acceleration;
        self.velocity.y += y_acceleration;
    }
}

#[derive(PartialEq)]
pub enum CelestialType {
    Planet,
    Asteroid,
}
