use crate::components::*;
use crate::state::GameState;
use bevy::prelude::*;

const GRAVITATIONAL_CONSTANT: f32 = 6.674e-11;

// TODO: Using this makes me feel like something is wrong somewhere
// Try refactoring match so this isn't needed
const SLOW_RATIO: f32 = 1e-12;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (process_physics, check_for_collisions)
                .chain()
                .run_if(in_state(GameState::InOrbit)),
        );
    }
}

fn process_physics(
    planets_query: Query<(&Transform, &Mass), With<Planet>>,
    mut asteroids_query: Query<(&mut Transform, &mut Velocity), With<Asteroid>>,
) {
    let (mut asteroid_transform, mut asteroid_velocity) = asteroids_query.single_mut();

    for (planet_transform, planet_mass) in planets_query.iter() {
        let distance = planet_transform
            .translation
            .distance(asteroid_transform.translation);
        let gravity = GRAVITATIONAL_CONSTANT * (planet_mass.0 / distance.powi(2));

        let dy = planet_transform.translation.y - asteroid_transform.translation.y;
        let dx = planet_transform.translation.x - asteroid_transform.translation.x;
        let theta = dy.atan2(dx);

        let x_acceleration = theta.cos() * gravity;
        let y_acceleration = theta.sin() * gravity;
        asteroid_velocity.accelerate(x_acceleration, y_acceleration);

        asteroid_transform.translation.x += asteroid_velocity.0.x * SLOW_RATIO;
        asteroid_transform.translation.y += asteroid_velocity.0.y * SLOW_RATIO;
    }
}

fn check_for_collisions(
    mut next_state: ResMut<NextState<GameState>>,
    planets_query: Query<(&Transform, &Radius), With<Planet>>,
    mut asteroids_query: Query<(&Transform, &mut Velocity, &Radius), With<Asteroid>>,
) {
    let (asteroid_transform, mut asteroid_velocity, asteroid_radius) = asteroids_query.single_mut();

    for (planet_transform, planet_radius) in planets_query.iter() {
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
