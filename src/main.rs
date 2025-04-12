use bevy::{
    DefaultPlugins,
    app::{App, Startup},
    core_pipeline::core_2d::Camera2d,
    ecs::{component::Component, system::Commands},
    hierarchy::DespawnRecursiveExt,
    prelude::*,
    state::{app::AppExtStates, state::States},
    window::{Window, WindowPlugin, WindowResolution},
};
use config::WINDOW_SIZE;
use ui::menu;

mod components;
mod config;
mod game;
mod systems;
mod ui;
mod utils;

#[derive(States, Debug, Default, Eq, PartialEq, Hash, Clone)]
pub enum GameState {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_SIZE.x, WINDOW_SIZE.y),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((game::game_plugin, menu::menu_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d,));
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
