use crate::{
    components::{CollisionEvent, Score},
    config::WINDOW_SIZE,
    systems::{collision::*, movement::*, scoring::setup_score, world::*},
};
use bevy::{prelude::*, window::WindowResolution};

pub fn init() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_SIZE.x, WINDOW_SIZE.y),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_event::<CollisionEvent>()
        .insert_resource(Score(0))
        .add_systems(
            Startup,
            (
                setup_game,
                setup_scene,
                setup_instructions,
                setup_camera,
                setup_score,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                move_player,
                move_ball,
                move_enemy,
                check_collisions,
                check_wall_collision,
            ),
        )
        .run();
}
