use std::sync::Arc;

use crate::{hittable::HitRecord, ray::Ray};

use super::MaterialRecord;

/// Trait for a material that can either absorb a ray or scatter it.
pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<MaterialRecord>;
}

/// A material trait object that can be shared between threads.
pub type ArcMaterial = Arc<dyn Material + Send + Sync>;
