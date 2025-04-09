use crate::{
    components::{Ball, Collider, CollisionEvent, Player, Velocity},
    config::GameConfig,
};
use bevy::{
    ecs::{
        entity::Entity,
        event::EventWriter,
        system::{Commands, ResMut, Single},
    },
    prelude::{ButtonInput, KeyCode, Query, Res, Time, Transform, Vec2, With},
};

pub fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    game_config: ResMut<GameConfig>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyJ) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyK) {
        direction.y += 1.;
    }

    let move_delta = direction.normalize_or_zero() * game_config.player_speed * time.delta_secs();
    player.translation += move_delta.extend(0.);
}

pub fn move_ball(ball: Single<(&mut Transform, &Velocity), With<Ball>>, time: Res<Time>) {
    let (mut ball_transform, ball_velocity) = ball.into_inner();

    ball_transform.translation.x += ball_velocity.0.x * time.delta_secs();
    ball_transform.translation.y += ball_velocity.0.y * time.delta_secs();
}

pub fn check_collisions(
    mut commands: Commands,
    ball: Single<(&mut Velocity, &Transform), With<Ball>>,
    colliders: Query<(Entity, &Transform), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    for (collider_entity, collider_transform) in &colliders {
        // TODO: https://bevyengine.org/examples/games/breakout/
    }
}
