use std::sync::Arc;

use crate::ray::Ray;

use itertools::Itertools;

use super::{hittables::NotHittable, ArcHittable, BoundingBox, HitRecord, Hittable, Interval};

/// A node of a Binary Volume Hierarchy tree.
///
/// Using BVH trees, the renderer doesn't need to check every object in the scene for a hit.
/// In the best case scenario, a BVH tree means a ray hit can be calculated in O(1) time, as long as the ray doesn't
/// actually hit anything. With actual use on my machine, I have seen speedups of over 10x using BVH trees. The
/// downside is that to calculate the best possible arrangement for the tree, the bounding box for every possible pair
/// of objects in the scene must be calculated at least once.
pub struct BvhNode {
    left: ArcHittable,
    right: ArcHittable,
    bounding_box: BoundingBox,
}

impl Into<ArcHittable> for BvhNode {
    fn into(self) -> ArcHittable {
        Arc::new(self)
    }
}

impl BvhNode {
    /// Construct a BVH tree from a collection of [ArcHittable] objects.
    pub fn create(objects: impl Into<Vec<ArcHittable>>) -> BvhNode {
        // Create a mutable copy of the objects vec to work with.
        let mut objects: Vec<ArcHittable> = objects.into();

        // Reduce the objects in the vec into nodes until there are two or less left.
        while objects.len() >= 2 {
            // Find the pair of objects in the scene with the smallest bounding box and create a node from them.
            let (node, _) = objects
                .clone()
                .into_iter()
                .tuple_combinations()
                .map(|(a, b)| {
                    let node = BvhNode::new(a, b);
                    let volume = node.bounding_box.volume();
                    (node, volume)
                })
                .reduce(|a, b| if a.1 < b.1 { a } else { b })
                .unwrap();

            // Remove the new node's children from objects and push the new node into objects.
            objects.retain(|x| !(Arc::ptr_eq(x, &node.left) || Arc::ptr_eq(x, &node.right)));
            objects.push(node.into());
        }

        // If there are two or less objects left, create a node from them.
        let (left, right) = match objects.len() {
            0 => (NotHittable.into(), NotHittable.into()),
            1 => (NotHittable.into(), Arc::clone(&objects[0])),
            2 => (Arc::clone(&objects[0]), Arc::clone(&objects[1])),
            _ => unreachable!(),
        };

        BvhNode::new(left, right)
    }

    /// Construct a new BVH node, ensuring that the left's volume <= right's volume.
    fn new(a: ArcHittable, b: ArcHittable) -> Self {
        let a_bb = a.bounding_box();
        let b_bb = b.bounding_box();
        let bounding_box = BoundingBox::containing(&a_bb, &b_bb);

        let (left, right) = if a_bb.volume() <= b_bb.volume() {
            (a, b)
        } else {
            (b, a)
        };

        Self {
            left,
            right,
            bounding_box,
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // Return early if the node's bounding box isn't hit.
        self.bounding_box.hit(ray, ray_t)?;

        match (self.left.hit(ray, ray_t), self.right.hit(ray, ray_t)) {
            // If only one hits, return the record for the hit.
            (Some(record), None) | (None, Some(record)) => Some(record),
            // If both hit, return the record with the smaller t value.
            (Some(record_a), Some(record_b)) => {
                if record_a.t < record_b.t {
                    Some(record_a)
                } else {
                    Some(record_b)
                }
            }
            // And if neither hit, return none.
            (None, None) => None,
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}
