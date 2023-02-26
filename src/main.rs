use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const GRAVITATIONAL_CONSTANT: f64 = 6.674e-11;

#[derive(Bundle)]
struct CelestialBundle {
    radius: Radius,
    celestial_type: CelestialType,
    movement: Movement,
    mass: Mass,
    #[bundle]
    sprite: MaterialMesh2dBundle<ColorMaterial>,
}

impl CelestialBundle {
    fn planet(
        radius: f32,
        mass: f64,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self::new(radius, CelestialType::Planet, mass, meshes, materials)
    }
    fn asteroid(
        radius: f32,
        mass: f64,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self::new(radius, CelestialType::Asteroid, mass, meshes, materials)
    }
    fn new(
        radius: f32,
        celestial_type: CelestialType,
        mass: f64,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self {
            radius: Radius(radius),
            celestial_type: celestial_type,
            movement: Movement {
                velocity: Vec2::new(0., 0.),
                acceleration: Vec2::new(0., 0.),
            },
            mass: Mass(mass),
            sprite: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(radius.into()).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..default()
            },
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Component)]
enum GameState {
    Following,
    FreeFall
}

#[derive(Component, PartialEq)]
enum CelestialType {
    Planet,
    Asteroid,
}

#[derive(Component)]
struct Mass(f64);

#[derive(Component)]
struct Movement {
    velocity: Vec2,
    acceleration: Vec2,
}

#[derive(Component)]
struct Radius(f32);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(CelestialBundle::planet(50., 5.972e25, &mut meshes, &mut materials));
    commands.spawn(CelestialBundle::asteroid(10., 5.972e25, &mut meshes, &mut materials));
}

fn handle_mouse_input(mut state: ResMut<State<GameState>>, buttons: Res<Input<MouseButton>>) {
    if buttons.just_pressed(MouseButton::Left) {
        // Left button was pressed
        println!("left pressed");
        state.set(GameState::FreeFall).unwrap();
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

fn handle_mouse_motion(state: Res<State<GameState>>, windows: Res<Windows>, mut query: Query<(&mut Transform, &CelestialType)>) {
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

fn handle_freefall() {
    // TOOD: physics things
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Following)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::on_update(GameState::Following)
                .with_system(handle_mouse_motion)
                .with_system(handle_mouse_input)
        )
        .add_system_set(
            SystemSet::on_update(GameState::FreeFall)
                .with_system(handle_freefall)
        )
        .run();
}
