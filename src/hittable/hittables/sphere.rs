use std::sync::Arc;

use crate::{
    hittable::{ArcHittable, HitRecord, Hittable},
    material::ArcMaterial,
    point3::Point3,
    ray::Ray,
};

/// So smooth and round!
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: ArcMaterial,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: &ArcMaterial) -> Self {
        Self {
            center,
            radius,
            material: Arc::clone(material),
        }
    }
}

impl Into<ArcHittable> for Sphere {
    fn into(self) -> ArcHittable {
        Arc::new(self)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;

        Some(HitRecord::new(
            p,
            t,
            ray,
            outward_normal,
            Arc::clone(&self.material),
        ))
    }
}
