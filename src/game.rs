pub mod game {
    use bevy::{core_pipeline::bloom::Bloom, prelude::*};

    use crate::components::components::{Enemy, Player};
    
    const WINDOW_SIZE: Vec2 = Vec2::new(1000., 700.);

    pub fn setup_scene(
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

}
