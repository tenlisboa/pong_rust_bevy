use bevy::{
    ecs::{component::Component, event::Event, system::Resource},
    math::Vec2,
    prelude::{Deref, DerefMut},
};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct ScoreBoard;

#[derive(Resource, Deref, DerefMut)]
pub struct Score(pub i32);

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}
