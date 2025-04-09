use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Ball {
    pub x_speed: f32,
    pub y_speed: f32,

    pub x: f32,
    pub y: f32,
}
