use bevy::{
    ecs::{
        event::EventWriter,
        query::With,
        system::{Query, Single},
    },
    math::{Vec2, bounding::Aabb2d},
    transform::components::Transform,
};

use crate::{
    components::{Ball, Collider, Collision, CollisionEvent, Velocity},
    config::{BALL_DIAMETER, WINDOW_SIZE},
    utils::collision,
};

pub fn check_collisions(
    // mut commands: Commands,
    ball: Single<(&mut Velocity, &Transform), With<Ball>>,
    colliders: Query<&Transform, With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, ball_transform) = ball.into_inner();

    for collider_transform in &colliders {
        let collision = collision::check(
            Aabb2d::new(
                ball_transform.translation.truncate(),
                Vec2::new(BALL_DIAMETER / 2., BALL_DIAMETER / 2.),
            ),
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

pub fn check_wall_collision(
    ball: Single<(&mut Velocity, &Transform), With<Ball>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, ball_transform) = ball.into_inner();
    let (half_x, half_y) = (WINDOW_SIZE.x / 2., WINDOW_SIZE.y / 2.);
    let (left, right, top, bottom) = (-half_x, half_x, half_y, -half_y);
    let (x, y) = (ball_transform.translation.x, ball_transform.translation.y);

    let ball_radius = BALL_DIAMETER / 2.;

    let has_hit_righ_wall = x + ball_radius > right;
    let has_hit_left_wall = x - ball_radius < left;
    let has_hit_top_wall = y + ball_radius > top;
    let has_hit_bottom_wall = y - ball_radius < bottom;

    if has_hit_righ_wall || has_hit_left_wall {
        collision_events.send_default();
        ball_velocity.0.x = -ball_velocity.0.x;
    }
    if has_hit_top_wall || has_hit_bottom_wall {
        collision_events.send_default();
        ball_velocity.0.y = -ball_velocity.0.y;
    }
}
