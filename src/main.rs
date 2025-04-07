use bevy::prelude::*;

const PLAYER_SPEED: f32 = 100.;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // World
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1000., 700.))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))),
    ));

    // Player
    commands.spawn((
        Player,
        Mesh2d(meshes.add(Circle::new(25.))),
        MeshMaterial2d(materials.add(Color::srgb(6.25, 9.4, 9.1))), // RGB values exceed 1 to achieve a bright color for the bloom effect
        Transform::from_xyz(0., 0., 2.),
    ));
}
