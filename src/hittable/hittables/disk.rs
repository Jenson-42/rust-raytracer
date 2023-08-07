use std::sync::Arc;

use crate::{
    hittable::{ArcHittable, HitRecord, Hittable},
    material::ArcMaterial,
    ray::Ray,
    Point3, Vec3,
};

/// Like a CD!
pub struct Disk {
    pub center: Point3,
    pub normal: Vec3<f64>,
    pub radius: f64,
    pub material: ArcMaterial,
}

impl Disk {
    pub fn new(center: Point3, normal: Vec3<f64>, radius: f64, material: &ArcMaterial) -> Self {
        Self {
            center,
            normal,
            radius,
            material: Arc::clone(material),
        }
    }
}

impl Into<ArcHittable> for Disk {
    fn into(self) -> ArcHittable {
        Arc::new(self)
    }
}

impl Hittable for Disk {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.direction);
        if denom.abs() < 1e-6 {
            return None;
        }

        let t = -(ray.origin.dot(&self.normal)) / denom;

        if t < t_min || t_max < t {
            return None;
        }

        let location = ray.at(t);
        if (location - self.center).length() > self.radius {
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
