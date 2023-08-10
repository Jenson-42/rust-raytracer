use std::sync::Arc;

use crate::{
    hittable::{ArcHittable, BoundingBox, HitRecord, Hittable, Interval},
    ray::Ray,
};

/// An unhittable unit struct. For internal use constructing BVH nodes where one or both children needs to be "empty",
/// without coercing the child type into an Option.
pub struct NotHittable;

impl Into<ArcHittable> for NotHittable {
    fn into(self) -> ArcHittable {
        Arc::new(NotHittable)
    }
}

impl Hittable for NotHittable {
    /// This will always return an empty bounding box.
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::empty()
    }

    /// This will always return false.
    fn hit(&self, _: &Ray, _: &Interval) -> Option<HitRecord> {
        None
    }
}
