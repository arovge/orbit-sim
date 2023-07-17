use bevy::prelude::*;

#[derive(Component)]
pub struct StateText;

#[derive(Component)]
pub struct CoordinatesText;

#[derive(Component)]
pub struct ModeText;

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

#[derive(Component, Default)]
pub struct Velocity(Vec3);

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
