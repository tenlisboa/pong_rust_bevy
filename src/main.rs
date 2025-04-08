use bevy::{prelude::*, window::WindowResolution};

mod components;

mod systems;
use systems::movements::move_player;

mod game;
use game::game::{setup_scene, setup_instructions, setup_camera};

const WINDOW_SIZE: Vec2 = Vec2::new(1000., 700.);

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

