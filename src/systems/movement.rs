use crate::{
    components::{Ball, Player},
    config::GameConfig,
};
use bevy::{
    ecs::system::ResMut,
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

pub fn move_ball(
    mut ball_transformer: Query<&mut Transform, With<Ball>>,
    ball: Query<&Ball>,
    time: Res<Time>,
    game_config: ResMut<GameConfig>,
) {
    let Ok(mut ball_transformer) = ball_transformer.get_single_mut() else {
        return;
    };

    let Ok(ball) = ball.get_single() else {
        return;
    };

    let direction = Vec2::new(ball.x, ball.y);

    let speed = Vec2::new(ball.x_speed, ball.y_speed);

    let move_delta = direction.normalize_or_zero() * speed * time.delta_secs();
    ball_transformer.translation += move_delta.extend(0.);
}
