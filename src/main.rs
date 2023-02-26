use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Bundle)]
struct CelestialBundle {
    radius: Radius,
    celestial_type: CelestialType,
    #[bundle]
    sprite: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Component, PartialEq)]
enum CelestialType {
    Planet,
    Asteroid
}

#[derive(Component)]
struct Radius(u32);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // TODO: Combine radius prop and radius reference on sprite mesh into one var
    commands.spawn(CelestialBundle {
        radius: Radius(50),
        celestial_type: CelestialType::Planet,
        sprite: MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        }
    });
    commands.spawn(CelestialBundle {
        radius: Radius(10),
        celestial_type: CelestialType::Asteroid,
        sprite: MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        }
    });
}

fn handle_mouse_input(
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        // Left button was pressed
        println!("left pressed");
    }
    if buttons.just_released(MouseButton::Left) {
        // Left Button was released
        println!("left released");
    }
    if buttons.pressed(MouseButton::Right) {
        // Right Button is being held down

        println!("right pressed");
    }
    // we can check multiple at once with `.any_*`
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        // Either the left or the right button was just pressed
        println!("left or right pressed");
    }
}

fn handle_mouse_motion(
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &CelestialType)>,
) {
    let window = windows.get_primary().unwrap();
    let half_width = window.width() / 2.;
    let half_height = window.height() / 2.;

    query
        .iter_mut()
        .filter(|q| *q.1 == CelestialType::Asteroid)
        .for_each(|mut q| {
            let Some(cursor_position) = window.cursor_position() else { return };
            q.0.translation.x = cursor_position.x - half_width;
            q.0.translation.y = cursor_position.y - half_height;
        });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(handle_mouse_input)
        .add_system(handle_mouse_motion)
        .run();
}
