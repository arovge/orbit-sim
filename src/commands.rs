use bevy::{
    asset::Assets,
    ecs::{system::Command, world::Mut},
    math::{primitives::Circle, Vec3},
    prelude::default,
    render::{
        color::Color,
        mesh::{Mesh, Meshable},
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    transform::components::Transform,
};

use crate::components::{Asteroid, Mass, Planet, Radius, Velocity};

const ASTEROID_RADIUS: f32 = 10.;
const PLANET_RADIUS: f32 = 50.;
const PLANET_MASS: f32 = 5.972e25;

pub enum CelestialBody {
    Asteroid,
    Planet { position: Vec3 },
}

impl CelestialBody {
    fn radius(&self) -> f32 {
        match self {
            CelestialBody::Asteroid => ASTEROID_RADIUS,
            CelestialBody::Planet { .. } => PLANET_RADIUS,
        }
    }
}

pub struct SpawnAsteroidCommand;

impl Command for SpawnAsteroidCommand {
    fn apply(self, world: &mut bevy::prelude::World) {
        apply_command(world, CelestialBody::Asteroid);
    }
}

pub struct SpawnPlanetCommand {
    pub position: Vec3,
}

impl Command for SpawnPlanetCommand {
    fn apply(self, world: &mut bevy::prelude::World) {
        apply_command(
            world,
            CelestialBody::Planet {
                position: self.position,
            },
        );
    }
}

fn apply_command(world: &mut bevy::prelude::World, celestial_body: CelestialBody) {
    let mesh_handle = world.resource_scope(|_world, mut meshes: Mut<Assets<Mesh>>| {
        let shape = Circle::new(celestial_body.radius());
        meshes.add(shape.mesh())
    });

    let material_handle =
        world.resource_scope(|_world, mut materials: Mut<Assets<ColorMaterial>>| {
            let material = ColorMaterial::from(Color::WHITE);
            materials.add(material)
        });

    let position = match celestial_body {
        CelestialBody::Asteroid => Vec3::ZERO,
        CelestialBody::Planet { position } => position,
    };

    let bundle = MaterialMesh2dBundle {
        mesh: mesh_handle.into(),
        material: material_handle,
        transform: Transform::from_translation(position),
        ..default()
    };

    match celestial_body {
        CelestialBody::Asteroid => {
            world.spawn((
                bundle,
                Asteroid,
                Radius(celestial_body.radius()),
                Velocity::default(),
            ));
        }
        CelestialBody::Planet { .. } => {
            world.spawn((
                bundle,
                Planet,
                Mass(PLANET_MASS),
                Radius(celestial_body.radius()),
            ));
        }
    }
}
