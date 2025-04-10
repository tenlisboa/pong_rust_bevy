use crate::components::{Ball, Enemy, Player, Velocity};
use bevy::{
    ecs::{query::Without, system::Single},
    prelude::{ButtonInput, KeyCode, Res, Time, Transform, Vec2, With},
};

pub fn move_player(
    player: Single<(&mut Transform, &Velocity), With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player_transform, player_velocity) = player.into_inner();

    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyJ) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyK) {
        direction.y += 1.;
    }

    let move_delta = direction.normalize_or_zero() * player_velocity.0 * time.delta_secs();
    player_transform.translation += move_delta.extend(0.);
}

pub fn move_ball(ball: Single<(&mut Transform, &Velocity), With<Ball>>, time: Res<Time>) {
    let (mut ball_transform, ball_velocity) = ball.into_inner();

    ball_transform.translation.x += ball_velocity.0.x * time.delta_secs();
    ball_transform.translation.y += ball_velocity.0.y * time.delta_secs();
}

pub fn move_enemy(
    enemy: Single<(&mut Transform, &Velocity), With<Enemy>>,
    ball: Single<&Transform, (With<Ball>, Without<Enemy>)>,
    time: Res<Time>,
) {
    let ball_transform = ball.into_inner();
    let (mut enemy_transform, enemy_velocity) = enemy.into_inner();

    let mut direction = Vec2::ZERO;

    if ball_transform.translation.y > enemy_transform.translation.y {
        direction.y += 1.;
    } else {
        direction.y -= 1.;
    }

    println!(
        "Direction: {} \nBall Pos: {} \nEnemy Pos: {} \nIs Ball Higher: {}",
        direction,
        ball_transform.translation,
        enemy_transform.translation,
        ball_transform.translation.y > enemy_transform.translation.y
    );

    let move_delta = direction.normalize_or_zero() * enemy_velocity.0 * time.delta_secs();
    enemy_transform.translation += move_delta.extend(0.);
}
