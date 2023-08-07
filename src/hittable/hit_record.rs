use crate::{material::ArcMaterial, ray::Ray, Point3, Vec3};

/// Struct representing the result of an object being hit.
pub struct HitRecord {
    /// The intersection's location in world space.
    pub hit_location: Point3,
    /// The normal of the face that was hit.
    pub normal: Vec3<f64>,
    /// The intersection point described as a fraction of the ray.
    pub t: f64,
    /// Whether the ray aligns with the normal of the face.
    pub front_face: bool,
    /// The material of the hit object.
    pub material: ArcMaterial,
}

impl HitRecord {
    /// Create a new HitRecord.
    pub fn new(
        p: Point3,
        t: f64,
        ray: &Ray,
        outward_normal: Vec3<f64>,
        material: ArcMaterial,
    ) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            hit_location: p,
            normal,
            t,
            front_face,
            material,
        }
    }
}
