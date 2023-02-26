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
    let planet_x = planet.0.translation.x;
    let planet_y = planet.0.translation.y;

    // TOOD: physics things
    query
        .iter_mut()
        .filter(|q| *q.1.celestial_type() == CelestialType::Asteroid)
        .for_each(|q| {
            println!("Planet x, y: {0}, {1}", planet_x, planet_y);
            println!(
                "Asteroid x, y: {0}, {1}",
                q.0.translation.x, q.0.translation.y
            );
            let x_distance = planet_x - q.0.translation.x;
            let y_distance = planet_y - q.0.translation.y;
            let euclidean_distance = (x_distance.powf(2.) + y_distance.powf(2.)).sqrt();
            println!("{euclidean_distance}");
            // TODO: Calculate acceleration based on distance to planet

            // TODO: Update velocity based on acceleration

            // TODO: Update position based on velocity
            // q.0.translation.x = q.1.
        });
}
