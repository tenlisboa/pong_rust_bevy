use bevy::{
    ecs::{
        event::EventWriter,
        query::With,
        system::{Query, Single},
    },
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    transform::components::Transform,
};

use crate::{
    components::{Ball, Collider, Collision, CollisionEvent, Velocity},
    config::BALL_DIAMETER,
};

pub fn check_collisions(
    // mut commands: Commands,
    ball: Single<(&mut Velocity, &Transform), With<Ball>>,
    colliders: Query<&Transform, With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, ball_transform) = ball.into_inner();

    for collider_transform in &colliders {
        let collision = ball_collision(
            BoundingCircle::new(ball_transform.translation.truncate(), BALL_DIAMETER / 2.),
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
