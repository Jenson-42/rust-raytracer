use crate::{ray::Ray, Point3};

use super::Interval;

/// An Axis-Aligned Bounding Box.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingBox {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl BoundingBox {
    /// Create a new bounding box that contains two points.
    /// ```
    /// # use raytracing_in_a_weekend::hittable::{BoundingBox, Interval};
    /// # use raytracing_in_a_weekend::Point3;
    /// let p1 = Point3::new(1.0, 3.0, 2.0);
    /// let p2 = Point3::new(0.0, 5.0, 1.0);
    ///
    /// let bb = BoundingBox::new(p1, p2);
    /// assert_eq!(bb.x, Interval::new(0.0, 1.0));
    /// assert_eq!(bb.y, Interval::new(3.0, 5.0));
    /// assert_eq!(bb.z, Interval::new(1.0, 2.0));
    /// ```
    pub fn new(a: Point3, b: Point3) -> Self {
        let x = Interval::new(a.x(), b.x());
        let y = Interval::new(a.y(), b.y());
        let z = Interval::new(a.z(), b.z());

        Self { x, y, z }
    }

    /// Create an empty bounding box.
    /// ```
    /// # use raytracing_in_a_weekend::hittable::{BoundingBox, Interval};
    /// # use raytracing_in_a_weekend::Point3;
    /// let bb = BoundingBox::empty();
    /// assert_eq!(bb.x, Interval::new(0.0, 0.0));
    /// assert_eq!(bb.y, Interval::new(0.0, 0.0));
    /// assert_eq!(bb.z, Interval::new(0.0, 0.0));
    pub fn empty() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    /// Return true if the bounding box is empty.
    pub fn is_empty(&self) -> bool {
        self.x.is_empty() && self.y.is_empty() && self.z.is_empty()
    }

    /// Check if a ray intersects with a bounding box, and if so, the interval ray_t at which it does.
    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<Interval> {
        // Return early if the bounding box is empty.
        if self.is_empty() {
            return None;
        }

        // We don't want to modify the original ray_t in case there isn't a hit.
        let mut ray_collision_t = ray_t.clone();

        let axes = [
            (&self.x, ray.origin.x(), ray.direction.x()),
            (&self.y, ray.origin.y(), ray.direction.y()),
            (&self.z, ray.origin.z(), ray.direction.z()),
        ];

        for (n_axis, n_ray_origin, n_ray_direction) in axes {
            // Get the collision interval of the ray for this axis.
            let n_collision_t = Interval::new(
                (n_axis.min - n_ray_origin) / n_ray_direction,
                (n_axis.max - n_ray_origin) / n_ray_direction,
            );

            // The portion of the ray that hits each axis must overlap to collide with the bounding box.
            ray_collision_t = Interval::overlap(ray_collision_t, n_collision_t)?;
        }

        Some(ray_collision_t)
    }

    /// Create a bounding box containing two other bounding box.
    pub fn containing(aabb0: &Self, aabb1: &Self) -> Self {
        let x = Interval::containing(aabb0.x, aabb1.x);
        let y = Interval::containing(aabb0.y, aabb1.y);
        let z = Interval::containing(aabb0.z, aabb1.z);

        Self { x, y, z }
    }

    /// Get the 3D volume of the bounding box.
    pub fn volume(&self) -> f64 {
        self.x.size() * self.y.size() * self.z.size()
    }
}
