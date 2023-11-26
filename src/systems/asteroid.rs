use crate::commands::SpawnAsteroidCommand;
use crate::components::*;
use crate::state::GameState;
use bevy::prelude::*;

const GRAVITATIONAL_CONSTANT: f32 = 6.674e-11;

// TODO: Using this makes me feel like something is wrong somewhere
// Try refactoring match so this isn't needed
const SLOW_RATIO: f32 = 1e-12;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            handle_asteroid_orbit.run_if(in_state(GameState::InOrbit)),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.add(SpawnAsteroidCommand);
}

fn handle_asteroid_orbit(
    mut next_state: ResMut<NextState<GameState>>,
    planets_query: Query<(&Transform, &Mass, &Radius), (With<Planet>, Without<Asteroid>)>,
    mut asteroids_query: Query<
        (&mut Transform, &mut Velocity, &Radius),
        (With<Asteroid>, Without<Planet>),
    >,
) {
    let asteroid = asteroids_query.single_mut();
    let (mut asteroid_transform, mut asteroid_velocity, asteroid_radius) = asteroid;

    for planet in planets_query.iter() {
        let (planet_transform, planet_mass, planet_radius) = planet;

        let distance = planet_transform
            .translation
            .distance(asteroid_transform.translation);
        let gravity = GRAVITATIONAL_CONSTANT * (planet_mass.mass() / distance.powi(2));

        let dy = planet_transform.translation.y - asteroid_transform.translation.y;
        let dx = planet_transform.translation.x - asteroid_transform.translation.x;
        let theta = dy.atan2(dx);

        let x_acceleration = theta.cos() * gravity;
        let y_acceleration = theta.sin() * gravity;
        asteroid_velocity.accelerate(x_acceleration, y_acceleration);

        asteroid_transform.translation.x += asteroid_velocity.velocity().x * SLOW_RATIO;
        asteroid_transform.translation.y += asteroid_velocity.velocity().y * SLOW_RATIO;

        // If the asteroid is touching a planet, reset the game
        let asteroid_distance_to_planet = planet_transform
            .translation
            .distance(asteroid_transform.translation);
        if asteroid_distance_to_planet < planet_radius.radius() + asteroid_radius.radius() {
            asteroid_velocity.reset();
            next_state.set(GameState::FollowingCursor);
        }
    }
}
