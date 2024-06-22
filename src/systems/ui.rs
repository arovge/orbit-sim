use crate::components::{Asteroid, CoordinatesText, Planet, StateText};
use crate::state::GameState;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (update_state_text, update_coordinates_text));
    }
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let font = asset_server.load("fonts/clacon2.ttf");
    let text_style = TextStyle {
        font,
        font_size: 18.0,
        color: Color::WHITE,
    };

    commands.spawn((
        TextBundle::from_section(GameState::FollowingCursor.to_string(), text_style.clone())
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(15.),
                left: Val::Px(15.),
                ..default()
            })
            .with_text_justify(JustifyText::Left),
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
            .with_text_justify(JustifyText::Right),
        CoordinatesText,
    ));
}

fn update_state_text(
    state: Res<State<GameState>>,
    mut text_query: Query<&mut Text, With<StateText>>,
) {
    let mut text = text_query.single_mut();
    text.sections[0].value = state.get().to_string();
}

fn update_coordinates_text(
    asteroid_query: Query<&Transform, (With<Asteroid>, Without<Planet>)>,
    mut text_query: Query<&mut Text, With<CoordinatesText>>,
) {
    let asteroid_translation = asteroid_query.single().translation;
    let mut text = text_query.single_mut();
    text.sections[0].value = format!(
        "{0:.2}, {1:.2}",
        asteroid_translation.x, asteroid_translation.y
    );
}
