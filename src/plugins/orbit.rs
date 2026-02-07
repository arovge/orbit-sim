use crate::components::*;
use crate::state::GameState;
use bevy::prelude::*;

use super::{WithAsteroid, WithPlanet};

const GRAVITATIONAL_CONST: f32 = 6.674e-11;

// TODO: Using this makes me feel like something is wrong somewhere
// Try refactoring math so this isn't needed
const SLOW_RATIO: f32 = 1e-12;

pub struct OrbitPlugin;

impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (tick_physics, check_for_collisions)
                .chain()
                .run_if(in_state(GameState::Orbit)),
        );
    }
}

fn tick_physics(
    planets: Query<(&Transform, &Mass), WithPlanet>,
    asteroid: Single<(&mut Transform, &mut Velocity), WithAsteroid>,
) {
    let (mut asteroid_transform, mut asteroid_velocity) = asteroid.into_inner();

    for (planet_transform, planet_mass) in planets.iter() {
        let distance = planet_transform
            .translation
            .distance(asteroid_transform.translation);
        let gravity = GRAVITATIONAL_CONST * (planet_mass.0 / distance.powi(2));

        let dy = planet_transform.translation.y - asteroid_transform.translation.y;
        let dx = planet_transform.translation.x - asteroid_transform.translation.x;
        let theta = dy.atan2(dx);

        let acceleration = Vec2::new(theta.cos(), theta.sin()) * gravity;
        asteroid_velocity.accelerate(acceleration);

        asteroid_transform.translation.x += asteroid_velocity.0.x * SLOW_RATIO;
        asteroid_transform.translation.y += asteroid_velocity.0.y * SLOW_RATIO;
    }
}

fn check_for_collisions(
    mut next_state: ResMut<NextState<GameState>>,
    planets: Query<(&Transform, &Radius), WithPlanet>,
    asteroid: Single<(&Transform, &mut Velocity, &Radius), WithAsteroid>,
) {
    let (asteroid_transform, mut asteroid_velocity, asteroid_radius) = asteroid.into_inner();

    for (planet_transform, planet_radius) in planets.iter() {
        let asteroid_distance_to_planet = planet_transform
            .translation
            .distance(asteroid_transform.translation);

        let is_colliding = asteroid_distance_to_planet < planet_radius.0 + asteroid_radius.0;

        if is_colliding {
            asteroid_velocity.reset();
            next_state.set(GameState::FollowingCursor);
        }
    }
}
