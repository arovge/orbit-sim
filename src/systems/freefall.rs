use crate::components::*;
use bevy::prelude::*;

const GRAVITATIONAL_CONSTANT: f64 = 6.674e-11;

pub fn handle_freefall(mut query: Query<(&mut Transform, &mut CelestialBody)>) {
    // TODO: This assumes there is ever only one planet celestial body
    // Change this whenever the program supports multiple planets
    let planet = query
        .iter()
        .find(|q| *q.1.celestial_type() == CelestialType::Planet)
        .unwrap();
    let planet_mass = planet.1.mass();
    let planet_x = planet.0.translation.x;
    let planet_y = planet.0.translation.y;

    // TOOD: physics things
    query
        .iter_mut()
        .filter(|q| *q.1.celestial_type() == CelestialType::Asteroid)
        .for_each(|mut q| {
            println!("Planet x, y: {0}, {1}", planet_x, planet_y);
            println!(
                "Asteroid x, y: {0}, {1}",
                q.0.translation.x, q.0.translation.y
            );
            let x_distance = planet_x - q.0.translation.x;
            let y_distance = planet_y - q.0.translation.y;
            let euclidean_distance = (x_distance.powf(2.) + y_distance.powf(2.)).sqrt() as f64;
            let gravity = -GRAVITATIONAL_CONSTANT * (planet_mass / euclidean_distance.powf(2.));
            let theta = q.0.translation.x.atan2(q.0.translation.y) as f64;
            
            // TODO: Everything below here could probably all be combined into
            // a function internal to CelestialBody instead of being here

            let new_x_acceleration = theta.sin() * gravity;
            let new_y_acceleration = theta.cos() * gravity;
            q.1.set_acceleration(new_x_acceleration, new_y_acceleration);

            let new_x_velocity = q.1.velocity().x;
            let new_y_velocity = q.1.velocity().y;
            q.1.set_velocity(new_x_velocity, new_y_velocity);

            // Update position based on velocity
            q.0.translation.x = q.1.velocity().x;
            q.0.translation.x = q.1.velocity().y;
        });
}
