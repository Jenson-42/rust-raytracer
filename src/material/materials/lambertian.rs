use std::sync::Arc;

use crate::{
    colour::Colour,
    hittable::HitRecord,
    material::{ArcMaterial, Material, MaterialRecord},
    ray::Ray,
    vec3::Vec3,
};

pub struct Lambertian {
    pub albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Into<ArcMaterial> for Lambertian {
    fn into(self) -> ArcMaterial {
        Arc::new(self)
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<MaterialRecord> {
        let scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        let scattered = Ray::new(hit_record.hit_location, scatter_direction);
        let attenuation = self.albedo;

        Some(MaterialRecord {
            scattered,
            attenuation,
        })
    }
}
