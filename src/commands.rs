use bevy::{
    asset::Assets,
    ecs::{system::Command, world::Mut},
    math::Vec3,
    prelude::default,
    render::{
        color::Color,
        mesh::{shape, Mesh},
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    transform::components::Transform,
};

use crate::components::{Asteroid, Mass, Planet, Radius, Velocity};

const ASTEROID_RADIUS: f32 = 10.;
const PLANET_RADIUS: f32 = 50.;
const PLANET_MASS: f32 = 5.972e25;

pub enum CelestialBodyKind {
    Asteroid,
    Planet(Vec3),
}

impl CelestialBodyKind {
    fn radius(&self) -> f32 {
        match self {
            CelestialBodyKind::Asteroid => ASTEROID_RADIUS,
            CelestialBodyKind::Planet(_) => PLANET_RADIUS,
        }
    }
}

pub struct SpawnAsteroidCommand;

impl Command for SpawnAsteroidCommand {
    fn apply(self, world: &mut bevy::prelude::World) {
        apply_command(world, CelestialBodyKind::Asteroid);
    }
}

pub struct SpawnPlanetCommand(pub Vec3);

impl Command for SpawnPlanetCommand {
    fn apply(self, world: &mut bevy::prelude::World) {
        apply_command(world, CelestialBodyKind::Planet(self.0));
    }
}

fn apply_command(world: &mut bevy::prelude::World, celestial_body_kind: CelestialBodyKind) {
    let mesh_handle = world.resource_scope(|_world, mut meshes: Mut<Assets<Mesh>>| {
        let shape = shape::Circle::new(celestial_body_kind.radius());
        meshes.add(Mesh::from(shape))
    });

    let material_handle =
        world.resource_scope(|_world, mut materials: Mut<Assets<ColorMaterial>>| {
            let material = ColorMaterial::from(Color::WHITE);
            materials.add(material)
        });

    match celestial_body_kind {
        CelestialBodyKind::Asteroid => {
            world.spawn((
                MaterialMesh2dBundle {
                    mesh: mesh_handle.into(),
                    material: material_handle,
                    transform: Transform::from_translation(Vec3::ZERO),
                    ..default()
                },
                Asteroid,
                Radius::new(celestial_body_kind.radius()),
                Velocity::default(),
            ));
        }
        CelestialBodyKind::Planet(position) => {
            world.spawn((
                MaterialMesh2dBundle {
                    mesh: mesh_handle.into(),
                    material: material_handle,
                    transform: Transform::from_translation(position),
                    ..default()
                },
                Planet,
                Mass::new(PLANET_MASS),
                Radius::new(celestial_body_kind.radius()),
            ));
        }
    }
}
