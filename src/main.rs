use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2d};

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
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(1000., 700.)).into(),
        material: materials.add(Color::rgb(0.2, 0.2, 0.3)),
        ..Default::default()
    });

    commands.spawn((
        Player,
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(25.)).into(),
            material: materials.add(Color::rgb(6.25, 9.4, 9.1)),
            transform: Transform::from_xyz(0., 0., 2.),
            ..Default::default()
        },
    ));
}
