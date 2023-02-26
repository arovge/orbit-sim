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

    pub fn celestial_type(&self) -> &CelestialType {
        &self.celestial_type
    }
}

#[derive(PartialEq)]
pub enum CelestialType {
    Planet,
    Asteroid,
}
