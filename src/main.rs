use bevy::{core_pipeline::bloom::Bloom, prelude::*, window::WindowResolution};

const PLAYER_SPEED: f32 = 300.;
const WINDOW_SIZE: Vec2 = Vec2::new(1000., 700.);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_SIZE.x, WINDOW_SIZE.y),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, (setup_scene, setup_instructions, setup_camera))
        .add_systems(Update, move_player)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // World
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(WINDOW_SIZE.x, WINDOW_SIZE.y))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))),
    ));

    // Player
    commands.spawn((
        Player,
        Mesh2d(meshes.add(Rectangle::new(20., 100.))),
        MeshMaterial2d(materials.add(Color::srgb(1.25, 2.4, 2.1))), // RGB values exceed 1 to achieve a bright color for the bloom effect
        Transform::from_xyz(-(WINDOW_SIZE.x / 2.) + 20., 0., 2.),
    ));

    // Enemy
    commands.spawn((
        Enemy,
        Mesh2d(meshes.add(Rectangle::new(20., 100.))),
        MeshMaterial2d(materials.add(Color::srgb(1.25, 0.4, 0.1))), // RGB values exceed 1 to achieve a bright color for the bloom effect
        Transform::from_xyz((WINDOW_SIZE.x / 2.) - 20., 0., 2.),
    ));
}

fn setup_instructions(mut commands: Commands) {
    commands.spawn((
        Text::new("Move use Vim motions J(Down) and K(Up)"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.),
            left: Val::Px(12.),
            ..Default::default()
        },
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true,
            ..Default::default()
        },
        Bloom::NATURAL,
    ));
}

fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // TODO: Research what is Ok
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyJ) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyK) {
        direction.y += 1.;
    } 

    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
    player.translation += move_delta.extend(0.);
}

