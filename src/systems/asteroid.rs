use crate::commands::SpawnAsteroidCommand;
use bevy::prelude::*;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.add(SpawnAsteroidCommand);
}
