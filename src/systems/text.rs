use crate::components::{Asteroid, CoordinatesText, Planet, StateText};
use crate::state::GameState;
use bevy::prelude::*;

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_text)
            .add_systems(Update, (update_state_text, update_coordinates_text));
    }
}

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/clacon2.ttf");
    let text_style = TextStyle {
        font,
        font_size: 18.0,
        color: Color::WHITE,
    };

    commands.spawn((
        TextBundle::from_section(GameState::FollowingCursor.description(), text_style.clone())
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(15.),
                left: Val::Px(15.),
                ..default()
            })
            .with_text_alignment(TextAlignment::Left),
        StateText,
    ));
    commands.spawn((
        TextBundle::from_section("0, 0", text_style)
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(15.),
                right: Val::Px(15.),
                ..default()
            })
            .with_text_alignment(TextAlignment::Right),
        CoordinatesText,
    ));
}

fn update_state_text(state: Res<State<GameState>>, mut query: Query<&mut Text, With<StateText>>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = state.get().description();
    }
}

fn update_coordinates_text(
    mut text_query: Query<&mut Text, With<CoordinatesText>>,
    asteroid_query: Query<&Transform, (With<Asteroid>, Without<Planet>)>,
) {
    let asteroid_translation = asteroid_query.single().translation;
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!(
            "{0:.2}, {1:.2}",
            asteroid_translation.x, asteroid_translation.y
        );
    }
}
