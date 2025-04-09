use bevy::{prelude::*, window::WindowResolution};
use crate::{config::WINDOW_SIZE, systems::world::*, systems::movement::*};

pub fn init() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_SIZE.x, WINDOW_SIZE.y),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, (setup_game, setup_scene, setup_instructions, setup_camera).chain())
        .add_systems(Update, (move_player, move_ball))
        .run();
}
