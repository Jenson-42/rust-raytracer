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
}

impl Checkerboard {
    pub fn new(mat1: ArcMaterial, mat2: ArcMaterial) -> Self {
        Self { mat1, mat2 }
    }
}

impl Into<ArcMaterial> for Checkerboard {
    fn into(self) -> ArcMaterial {
        Arc::new(self)
    }
}

impl Material for Checkerboard {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialRecord> {
        let Vec3(x, _, z) = hit_record.hit_location;
        if ((x.abs() + 0.5).floor() % 2.0 == 0.0) ^ ((z.abs() + 0.5).floor() % 2.0 == 0.0) {
            self.mat1.scatter(ray, hit_record)
        } else {
            self.mat2.scatter(ray, hit_record)
        }
    }
}
