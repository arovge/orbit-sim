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
    velocity: Vec2,
    acceleration: Vec2,
}

impl CelestialBody {
    pub fn new(radius: f32, mass: f64, celestial_type: CelestialType) -> Self {
        Self {
            radius,
            mass,
            celestial_type,
            velocity: Vec2::new(0., 0.),
            acceleration: Vec2::new(0., 0.),
        }
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn celestial_type(&self) -> &CelestialType {
        &self.celestial_type
    }

    pub fn velocity(&self) -> &Vec2 {
        &self.velocity
    }

    pub fn set_velocity(&mut self, x: f32, y: f32) {
        self.velocity.x = x;
        self.velocity.y = y;
    }

    pub fn acceleration(&self) -> &Vec2 {
        &self.acceleration
    }

    pub fn set_acceleration(&mut self, x: f64, y: f64) {
        self.acceleration.x = x as f32;
        self.acceleration.y = y as f32;
    }
}

#[derive(PartialEq)]
pub enum CelestialType {
    Planet,
    Asteroid,
}
