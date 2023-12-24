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
pub struct Radius(pub f32);

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn accelerate(&mut self, x_acceleration: f32, y_acceleration: f32) {
        self.0.x += x_acceleration;
        self.0.y += y_acceleration;
    }

    pub fn reset(&mut self) {
        self.0 = Vec2::ZERO;
    }
}

#[derive(Component)]
pub struct Planet;

#[derive(Component)]
pub struct Mass(pub f32);
