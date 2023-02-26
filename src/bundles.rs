use crate::components::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Bundle)]
pub struct CelestialBundle {
    body: CelestialBody,
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
            body: CelestialBody::new(radius, mass, celestial_type),
            sprite: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(radius.into()).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..default()
            },
        }
    }
}
