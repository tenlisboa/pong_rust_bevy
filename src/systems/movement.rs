use crate::{
    components::{Ball, Collider, Collision, CollisionEvent, Player, Velocity},
    config::GameConfig,
};
use bevy::{
    ecs::{
        event::EventWriter,
        system::{ResMut, Single},
    },
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
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
    // mut commands: Commands,
    ball: Single<(&mut Velocity, &Transform), With<Ball>>,
    colliders: Query<&Transform, With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, ball_transform) = ball.into_inner();

    for collider_transform in &colliders {
        let collision = ball_collision(
            BoundingCircle::new(ball_transform.translation.truncate(), 30. / 2.),
            Aabb2d::new(
                collider_transform.translation.truncate(),
                collider_transform.scale.truncate() / 2.,
            ),
        );

        if let Some(collision) = collision {
            collision_events.send_default();
            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = ball_velocity.0.x > 0.,
                Collision::Right => reflect_x = ball_velocity.0.x < 0.,
                Collision::Top => reflect_y = ball_velocity.0.y < 0.,
                Collision::Bottom => reflect_y = ball_velocity.0.y > 0.,
            }

            if reflect_x {
                ball_velocity.0.x = -ball_velocity.0.x;
            }

            if reflect_y {
                ball_velocity.0.y = -ball_velocity.0.y;
            }
        }
    }
}

fn ball_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&bounding_box) {
        return None;
    }

    let closest = bounding_box.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
