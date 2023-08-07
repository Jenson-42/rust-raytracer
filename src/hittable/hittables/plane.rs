use std::sync::Arc;

use crate::{
    hittable::{ArcHittable, HitRecord, Hittable},
    material::ArcMaterial,
    ray::Ray,
    Point3, Vec3,
};

/// An infinitely large plane.
pub struct Plane {
    pub center: Point3,
    pub normal: Vec3<f64>,
    pub material: ArcMaterial,
}

impl Plane {
    pub fn new(center: Point3, normal: Vec3<f64>, material: &ArcMaterial) -> Self {
        Self {
            center,
            normal,
            material: Arc::clone(material),
        }
    }
}

impl Into<ArcHittable> for Plane {
    fn into(self) -> ArcHittable {
        Arc::new(self)
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.direction);
        if denom.abs() < 1e-6 {
            return None;
        }

        let t = -(ray.origin.dot(&self.normal)) / denom;

        if t < t_min || t_max < t {
            return None;
        }

        Some(HitRecord::new(
            ray.at(t),
            t,
            ray,
            self.normal,
            Arc::clone(&self.material),
        ))
    }
}
