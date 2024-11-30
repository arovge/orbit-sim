use crate::components::{CoordinatesText, StateText};
use crate::state::GameState;
use bevy::prelude::*;

use super::WithAsteroid;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (update_state_text, update_coordinates_text));
    }
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let font = asset_server.load("fonts/clacon2.ttf");
    let text_font = TextFont {
        font,
        font_size: 18.,
        ..default()
    };
    let text_color = TextColor(Color::WHITE);

    commands.spawn((
        Text::new(GameState::FollowingCursor.to_string()),
        text_font.clone(),
        text_color,
        TextLayout {
            justify: JustifyText::Left,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(15.),
            left: Val::Px(15.),
            ..default()
        },
        StateText,
    ));
    commands.spawn((
        Text::new("0, 0"),
        text_font,
        text_color,
        TextLayout {
            justify: JustifyText::Right,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(15.),
            right: Val::Px(15.),
            ..default()
        },
        CoordinatesText,
    ));
}

fn update_state_text(
    state: Res<State<GameState>>,
    mut text_query: Query<&mut Text, With<StateText>>,
) {
    let mut text = text_query.single_mut();
    text.0 = state.get().to_string();
}

fn update_coordinates_text(
    asteroid_query: Query<&Transform, WithAsteroid>,
    mut text_query: Query<&mut Text, With<CoordinatesText>>,
) {
    let asteroid_translation = asteroid_query.single().translation;
    let mut text = text_query.single_mut();
    text.0 = format!(
        "{0:.2}, {1:.2}",
        asteroid_translation.x, asteroid_translation.y
    );
}
