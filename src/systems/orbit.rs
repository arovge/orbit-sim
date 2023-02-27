use crate::components::*;
use bevy::prelude::*;

const GRAVITATIONAL_CONSTANT: f64 = 6.674e-11;

// TODO: Using this makes me feel like something is wrong somewhere
// Try refactoring match so this isn't needed
const SLOW_RATIO: f32 = 1e-12;

pub fn handle_orbit(
    mut state: ResMut<State<GameState>>,
    planets_query: Query<(&Transform, &Planet), Without<Asteroid>>,
    mut asteroids_query: Query<(&mut Transform, &mut Asteroid), Without<Planet>>,
) {
    for planet in planets_query.iter() {
        asteroids_query.iter_mut().for_each(|mut asteroid| {
            let distance = planet.0.translation.distance(asteroid.0.translation);
            let gravity = GRAVITATIONAL_CONSTANT * (planet.1.mass() / distance.powi(2) as f64);

            let dy = planet.0.translation.y - asteroid.0.translation.y;
            let dx = planet.0.translation.x - asteroid.0.translation.x;
            let theta = f32::atan2(dy, dx) as f64;

            let new_x_acceleration = theta.cos() * gravity;
            let new_y_acceleration = theta.sin() * gravity;
            asteroid
                .1
                .update_physics(new_x_acceleration as f32, new_y_acceleration as f32);

            // Update position based on new velocity
            asteroid.0.translation.x += asteroid.1.velocity().x * SLOW_RATIO;
            asteroid.0.translation.y += asteroid.1.velocity().y * SLOW_RATIO;

            // If the asteroid is touching the planet, reset the game
            let asteroid_distance_to_planet =
                planet.0.translation.distance(asteroid.0.translation) as f64;
            if asteroid_distance_to_planet < planet.1.radius() + asteroid.1.radius() {
                asteroid.1.reset();
                _ = state.set(GameState::FollowingCursor);
            }
        });
    }
}
