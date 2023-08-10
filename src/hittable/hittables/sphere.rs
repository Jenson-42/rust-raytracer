use std::sync::Arc;

use crate::{
    hittable::{bounding_box::BoundingBox, ArcHittable, HitRecord, Hittable, Interval},
    material::ArcMaterial,
    point3::Point3,
    ray::Ray,
    Vec3,
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
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
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
        if root < ray_t.min || ray_t.max < root {
            root = (-half_b + sqrtd) / a;
            if root < ray_t.min || ray_t.max < root {
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

    fn bounding_box(&self) -> BoundingBox {
        let r_vec = Vec3::new(self.radius, self.radius, self.radius);
        BoundingBox::new(self.center - r_vec, self.center + r_vec)
    }
}
