use bevy::{
    asset::Assets,
    color::Color,
    ecs::world::{Command, Mut},
    math::{primitives::Circle, Vec3},
    prelude::Mesh2d,
    render::mesh::{Mesh, Meshable},
    sprite::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use crate::components::{Asteroid, Mass, Planet, Radius};

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

    let props = (
        Mesh2d(mesh_handle),
        MeshMaterial2d(material_handle),
        Transform::from_translation(position),
        Radius(celestial_body.radius()),
    );

    match celestial_body {
        CelestialBody::Asteroid => {
            world.spawn((props, Asteroid));
        }
        CelestialBody::Planet { .. } => {
            world.spawn((props, Planet, Mass(PLANET_MASS)));
        }
    }
}
