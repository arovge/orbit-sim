use crate::components::*;
use bevy::prelude::*;

const GRAVITATIONAL_CONSTANT: f64 = 6.674e-11;

// TODO: Using this makes me feel like something is wrong somewhere
// Try refactoring match so this isn't needed
const SLOW_RATIO: f32 = 1e-12;

pub fn handle_orbit(
    mut state: ResMut<State<GameState>>,
    planets_query: Query<(&Transform, &Planet), Without<Asteroid>>,
    mut asteroids_query: Query<(&mut Transform, &mut Asteroid), Without<Planet>>
) {
    // TODO: This assumes there is ever only one planet celestial body
    // Change this whenever the program supports multiple planets
    let planet = planets_query.single();
    let planet_mass = planet.1.mass();
    let planet_radius = planet.1.radius();
    let planet_translation = planet.0.translation;

    asteroids_query
        .iter_mut()
        .for_each(|mut q| {
            let euclidean_distance = planet_translation.distance(q.0.translation);
            let gravity = -GRAVITATIONAL_CONSTANT * (planet_mass / euclidean_distance.powf(2.) as f64);
            let theta = (q.0.translation.x).atan2(q.0.translation.y) as f64;

            // TOOD: If CelestialBody 'owned' it's position, this could be simplified
            let new_x_acceleration = theta.sin() * gravity;
            let new_y_acceleration = theta.cos() * gravity;
            q.1.update_physics(new_x_acceleration as f32, new_y_acceleration as f32);

            // Update position based on new velocity
            q.0.translation.x += q.1.velocity().x * SLOW_RATIO;
            q.0.translation.y += q.1.velocity().y * SLOW_RATIO;

            // If the asteroid is touching the planet, reset the game
            let asteroid_distance_to_planet = planet_translation.distance(q.0.translation) as f64;
            if asteroid_distance_to_planet < planet_radius + q.1.radius() {
                q.1.reset();
                state.set(GameState::FollowingCursor).unwrap();
            }
        });
}
