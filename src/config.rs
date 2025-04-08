use bevy::{ecs::system::Resource, math::Vec2};

pub const WINDOW_SIZE: Vec2 = Vec2::new(1000., 700.);

#[derive(Resource)]
pub struct GameConfig {
    pub window_size: Vec2,
    pub player_speed: f32,
    pub enemy_speed: f32,
}
