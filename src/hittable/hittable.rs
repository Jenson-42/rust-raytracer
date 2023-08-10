use std::sync::Arc;

use crate::ray::Ray;

use super::{bounding_box::BoundingBox, HitRecord, Interval};

/// A trait for any struct that can be intersected by a [Ray].
///
/// Note: In order to be used by the renderer, the struct must also implement [Send] and [Sync].
pub trait Hittable {
    /// Return a [HitRecord] of the closest point `t` along a [Ray] where `t_min < t < t_max` intersects.
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> BoundingBox;
}

// Implement hittable for a vector of objects that implement hittable.
impl Hittable for Vec<ArcHittable> {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // We only need the hit record of the closest objcect.
        let mut closest_so_far = ray_t.max;
        let mut hit = None;

        // Loop through each object in the vec and find the closest hit (if any).
        for object in self.iter() {
            if let Some(record) = object.hit(
                ray,
                &Interval {
                    min: ray_t.min,
                    max: closest_so_far,
                },
            ) {
                closest_so_far = record.t;
                hit = Some(record);
            }
        }

        hit
    }

    fn bounding_box(&self) -> BoundingBox {
        let mut ix = Interval::new(0.0, 0.0);
        let mut iy = Interval::new(0.0, 0.0);
        let mut iz = Interval::new(0.0, 0.0);

        for object in self.iter() {
            ix = Interval::containing(object.bounding_box().x, ix);
            iy = Interval::containing(object.bounding_box().y, iy);
            iz = Interval::containing(object.bounding_box().z, iz);
        }

        BoundingBox {
            x: ix,
            y: iy,
            z: iz,
        }
    }
}

/// A Hittable trait object that can be shared between threads.
pub type ArcHittable = Arc<dyn Hittable + Send + Sync>;
