use std::sync::Arc;

use crate::{
    colour::Colour,
    hittable::HitRecord,
    material::{ArcMaterial, Material, MaterialRecord},
    ray::Ray,
    vec3::Vec3,
};

pub struct Metal {
    pub albedo: Colour,
    pub fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Colour, fuzziness: f64) -> Self {
        Self { albedo, fuzziness }
    }
}

impl Into<ArcMaterial> for Metal {
    fn into(self) -> ArcMaterial {
        Arc::new(self)
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialRecord> {
        let reflected = ray.direction.unit_vector().reflect(&hit_record.normal);
        let scattered = Ray::new(
            hit_record.hit_location,
            reflected + Vec3::random_in_unit_sphere() * self.fuzziness,
        );
        let attenuation = self.albedo;

        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some(MaterialRecord {
                attenuation,
                scattered,
            })
        } else {
            None
        }
    }
}
