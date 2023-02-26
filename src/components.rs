use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub mod components {
    pub use crate::components::*;
}

#[derive(Bundle)]
pub struct CelestialBundle {
    radius: Radius,
    celestial_type: CelestialType,
    movement: Movement,
    mass: Mass,
    #[bundle]
    sprite: MaterialMesh2dBundle<ColorMaterial>,
}

impl CelestialBundle {
    pub fn planet(
        radius: f32,
        mass: f64,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self::new(radius, CelestialType::Planet, mass, meshes, materials)
    }
    pub fn asteroid(
        radius: f32,
        mass: f64,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self::new(radius, CelestialType::Asteroid, mass, meshes, materials)
    }
    pub fn new(
        radius: f32,
        celestial_type: CelestialType,
        mass: f64,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self {
            radius: Radius(radius),
            celestial_type: celestial_type,
            movement: Movement {
                velocity: Vec2::new(0., 0.),
                acceleration: Vec2::new(0., 0.),
            },
            mass: Mass(mass),
            sprite: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(radius.into()).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..default()
            },
        }
    }
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

#[derive(Component)]
pub struct Movement {
    velocity: Vec2,
    acceleration: Vec2,
}

#[derive(Component)]
pub struct Radius(f32);
