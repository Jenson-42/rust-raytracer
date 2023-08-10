use std::sync::Arc;

use crate::{
    hittable::HitRecord,
    material::{ArcMaterial, Material, MaterialRecord},
    ray::Ray,
    Vec3,
};

/// A checkerboard material that chooses between two other materials.
pub struct Checkerboard {
    pub mat1: ArcMaterial,
    pub mat2: ArcMaterial,
    pub scale: f64,
}

impl Checkerboard {
    pub fn new(scale: f64, mat1: ArcMaterial, mat2: ArcMaterial) -> Self {
        Self { mat1, mat2, scale }
    }
}

impl Into<ArcMaterial> for Checkerboard {
    fn into(self) -> ArcMaterial {
        Arc::new(self)
    }
}

impl Material for Checkerboard {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialRecord> {
        let Vec3(x, y, z) = hit_record.hit_location;
        let compute = |n: f64| f64::floor(f64::abs(n + 0.5) / self.scale) % 2.0 == 0.0;
        if compute(x) ^ compute(y) ^ compute(z) {
            self.mat1.scatter(ray, hit_record)
        } else {
            self.mat2.scatter(ray, hit_record)
        }
    }
}
