use std::sync::Arc;

use crate::ray::Ray;

use super::HitRecord;

/// A trait for any struct that can be intersected by a [Ray].
///
/// Note: In order to be used by the renderer, the struct must also implement [Send] and [Sync].
pub trait Hittable {
    /// Return a [HitRecord] of the closest point `t` along a [Ray] where `t_min < t < t_max` intersects.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

// Implement hittable for a vector of objects that implement hittable.
impl Hittable for Vec<ArcHittable> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // We only need the hit record of the closest objcect.
        let mut closest_so_far = t_max;
        let mut hit = None;

        // Loop through each object in the vec and find the closest hit (if any).
        for object in self.iter() {
            if let Some(record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t;
                hit = Some(record);
            }
        }

        hit
    }
}

/// A Hittable trait object that can be shared between threads.
pub type ArcHittable = Arc<dyn Hittable + Send + Sync>;
