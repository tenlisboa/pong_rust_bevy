use bevy::{core_pipeline::bloom::Bloom, prelude::*};
use rand::Rng;

use crate::components::{Ball, Collider, Enemy, Player, Velocity};
use crate::config::{BALL_DIAMETER, GameConfig, PADDLE_SIZE};

pub fn setup_game(mut commands: Commands) {
    commands.insert_resource(GameConfig {
        window_size: Vec2::new(1000., 700.),
        ball_speed: 300.,
        player_speed: 300.,
        enemy_speed: 150.,
    });
}

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_config: ResMut<GameConfig>,
) {
    // World
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(
            game_config.window_size.x,
            game_config.window_size.y,
        ))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))),
    ));

    // Player
    commands.spawn((
        Player,
        Mesh2d(meshes.add(Rectangle::new(PADDLE_SIZE.x, PADDLE_SIZE.y))),
        MeshMaterial2d(materials.add(Color::srgb(1.25, 2.4, 2.1))),
        Transform::from_xyz(-(game_config.window_size.x / 2.) + 20., 0., 2.),
        Velocity(Vec2::new(0., 1.).normalize() * game_config.player_speed),
        Collider,
    ));

    // Enemy
    commands.spawn((
        Enemy,
        Mesh2d(meshes.add(Rectangle::new(PADDLE_SIZE.x, PADDLE_SIZE.y))),
        MeshMaterial2d(materials.add(Color::srgb(1.25, 0.4, 0.1))),
        Transform::from_xyz((game_config.window_size.x / 2.) - 20., 0., 2.),
        Velocity(Vec2::new(0., 1.).normalize() * game_config.enemy_speed),
        Collider,
    ));

    // Ball
    let random_direction = Vec2::new(rand::rng().random::<f32>(), rand::rng().random::<f32>());
    commands.spawn((
        Ball,
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::srgb(1.25, 1.25, 1.25))),
        Transform::from_xyz(0., 0., 2.).with_scale(Vec2::splat(BALL_DIAMETER).extend(1.)),
        Velocity(random_direction.normalize() * game_config.ball_speed),
    ));
}

pub fn setup_instructions(mut commands: Commands) {
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

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true,
            ..Default::default()
        },
        Bloom::NATURAL,
    ));
}
