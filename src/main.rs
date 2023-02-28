mod components;
mod resources;
mod systems;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use components::{components::StateText, *};
use resources::*;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

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
                position: UiRect {
                    bottom: Val::Px(15.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            })
            .with_text_alignment(TextAlignment::BOTTOM_LEFT),
        StateText,
    ));
    // TOOD: Support dynamically adding more planets
    // commands.spawn((
    //     MaterialMesh2dBundle {
    //         mesh: meshes.add(shape::Circle::new(50.).into()).into(),
    //         material: materials.add(ColorMaterial::from(Color::WHITE)),
    //         transform: Transform::from_translation(Vec3::new(-350., 150., 0.)),
    //         ..default()
    //     },
    //     Planet,
    //     Mass::new(5.972e25),
    //     Radius::new(50.)
    // ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Planet,
        Mass::new(5.972e25),
        Radius::new(50.),
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Asteroid,
        Radius::new(10.),
        Velocity::default(),
    ));
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_state(GameState::FollowingCursor)
        .init_resource::<MouseDragResource>()
        .add_startup_system(setup)
        .add_system(systems::check_for_exit_key_press)
        .add_system(systems::update_state_text)
        .add_system_set(
            SystemSet::on_update(GameState::FollowingCursor)
                .with_system(systems::handle_cursor_moved)
                .with_system(systems::handle_asteroid_drag_start),
        )
        .add_system_set(
            SystemSet::on_update(GameState::CursorDragStarted)
                .with_system(systems::check_for_reset_key_press)
                .with_system(systems::handle_asteroid_drag_end),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InOrbit)
                .with_system(systems::handle_orbit)
                .with_system(systems::check_for_reset_key_press),
        )
        .run();
}
