use super::{Aabb, HitRecord, Object};
use crate::core::Ray;

use std::cmp::Ordering;
use std::sync::Arc;

use rand::prelude::*;

pub struct BVHNode {
    left: Arc<dyn Object>,
    right: Arc<dyn Object>,
    boxx: Aabb,
}

impl BVHNode {
    pub fn new(
        objects: &mut [Arc<dyn Object>],
        t_min: f64,
        t_max: f64,
        rng: &mut ThreadRng,
    ) -> Self {
        let axis = rng.gen_range(0, 3);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };

        let object_span = objects.len();

        let left;
        let right;

        if object_span == 1 {
            left = objects[0].clone();
            right = objects[0].clone();
        } else if object_span == 2 {
            if comparator(&objects[0], &objects[1]) == Ordering::Less {
                left = objects[0].clone();
                right = objects[1].clone();
            } else {
                left = objects[1].clone();
                right = objects[0].clone();
            }
        } else {
            objects.sort_by(comparator);
            let mid = object_span / 2;
            left = Arc::new(Self::new(&mut objects[0..mid], t_min, t_max, rng));
            right = Arc::new(Self::new(&mut objects[mid..], t_min, t_max, rng));
        }

        let box_left = left.bounding_box(t_min, t_max);
        let box_right = right.bounding_box(t_min, t_max);

        let boxx = Aabb::surrounding_box(&box_left, &box_right);

        Self { left, right, boxx }
    }
}

impl Object for BVHNode {
    fn hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
        rng: &mut ThreadRng,
    ) -> bool {
        if !self.boxx.hit(ray, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(ray, t_min, t_max, rec, rng);
        let hit_right = self
            .right
            .hit(ray, t_min, if hit_left { rec.t } else { t_max }, rec, rng);

        hit_left || hit_right
    }

    fn bounding_box(&self, _t_min: f64, _t_max: f64) -> Aabb {
        self.boxx.clone()
    }
}

fn box_compare(a: &Arc<dyn Object>, b: &Arc<dyn Object>, axis: usize) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);

    box_a.min[axis].partial_cmp(&box_b.min[axis]).unwrap()
}

fn box_x_compare(a: &Arc<dyn Object>, b: &Arc<dyn Object>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Object>, b: &Arc<dyn Object>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Object>, b: &Arc<dyn Object>) -> Ordering {
    box_compare(a, b, 2)
}
