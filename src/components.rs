use bevy::prelude::*;

#[derive(Component)]
#[require(Velocity)]
pub struct Asteroid;

#[derive(Component)]
pub struct Radius(pub f32);

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn accelerate(&mut self, acceleration: Vec2) {
        self.0 += acceleration;
    }

    pub fn reset(&mut self) {
        self.0 = Vec2::ZERO;
    }
}

#[derive(Component)]
pub struct Planet;

#[derive(Component)]
pub struct Mass(pub f32);
