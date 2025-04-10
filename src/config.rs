use bevy::{ecs::system::Resource, math::Vec2};

pub const WINDOW_SIZE: Vec2 = Vec2::new(1000., 700.);
pub const BALL_DIAMETER: f32 = 40.;
pub const PADDLE_SIZE: Vec2 = Vec2::new(20., 100.);

#[derive(Resource)]
pub struct GameConfig {
    pub window_size: Vec2,
    pub ball_speed: f32,
    pub player_speed: f32,
    pub enemy_speed: f32,
}
