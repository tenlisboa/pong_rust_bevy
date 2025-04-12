use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};

use crate::components::Collision;

pub fn check(entity_a: Aabb2d, entity_b: Aabb2d) -> Option<Collision> {
    if !entity_a.intersects(&entity_b) {
        return None;
    }

    let closest = entity_b.closest_point(entity_a.center());
    let offset = entity_a.center() - closest;
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
