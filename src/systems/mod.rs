use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::{Asteroid, Planet};

pub mod asteroid_drag;
pub mod input;
pub mod physics;
pub mod planet_editor;
pub mod ui;

/// A [`bevy::ecs::query::QueryFilter`] to ensure a query's access to [`Planet`] is disjoint from access to [`Asteroid`].
pub type WithPlanet = (With<Planet>, Without<Asteroid>);

/// A [`bevy::ecs::query::QueryFilter`] to ensure a query's access to [`Asteroid`] is disjoint from access to [`Planet`].
pub type WithAsteroid = (With<Asteroid>, Without<Planet>);

/// Translates the cursor position on the window into its position in the world.
///
/// Returns [`None`] if the cursor position is not on the primary window.
pub fn world_position_2d(
    window_query: &Query<&Window, With<PrimaryWindow>>,
    camera_query: &Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) -> Option<Vec2> {
    let cursor_position = window_query.single().ok()?.cursor_position()?;
    let (camera, camera_transform) = camera_query.single().ok()?;
    camera
        .viewport_to_world_2d(camera_transform, cursor_position)
        .ok()
}
