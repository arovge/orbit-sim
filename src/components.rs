use bevy::prelude::*;

pub mod components {
    pub use crate::components::*;
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Component)]
pub enum GameState {
    Following,
    FreeFall,
}

#[derive(Component, PartialEq)]
pub enum CelestialType {
    Planet,
    Asteroid,
}

#[derive(Component)]
pub struct Mass(f64);

impl Mass {
    pub fn new(mass: f64) -> Self {
        Self(mass)
    }
}

#[derive(Component)]
pub struct Movement {
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl Movement {
    pub fn new(velocity: Vec2, acceleration: Vec2) -> Self {
        Self { velocity, acceleration }
    }
}

#[derive(Component)]
pub struct Radius(f32);

impl Radius {
    pub fn new(radius: f32) -> Self {
        Self(radius)
    }
}
