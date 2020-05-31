use super::{Aabb, HitRecord, Hittable};
use crate::core::{Point, Ray};
use crate::utils::random_int;

use std::cmp::Ordering;
use std::sync::Arc;

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    boxx: Aabb,
}

impl BVHNode {
    pub fn new(objects: &mut [Arc<dyn Hittable>], t_min: f64, t_max: f64) -> Self {
        let axis = random_int(0, 3);
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
            left = Arc::new(Self::new(&mut objects[0..mid], t_min, t_max));
            right = Arc::new(Self::new(&mut objects[mid..], t_min, t_max));
        }

        let mut box_left = Aabb::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 0.0));
        let mut box_right = Aabb::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 0.0));

        if !left.bounding_box(t_min, t_max, &mut box_left)
            || !right.bounding_box(t_min, t_max, &mut box_right)
        {
            eprintln!("Not bounding box in BVHNode constructor");
        }

        let boxx = Aabb::surrounding_box(&box_left, &box_right);

        Self { left, right, boxx }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.boxx.hit(ray, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(ray, t_min, t_max, rec);
        let hit_right = self
            .right
            .hit(ray, t_min, if hit_left { rec.t } else { t_max }, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self, _t_min: f64, _t_max: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.boxx.clone();
        true
    }
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
    let mut box_a = Aabb::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 0.0));
    let mut box_b = Aabb::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 0.0));

    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        eprintln!("No bounding box in BVHNode constructor.");
    }

    box_a.min[axis].partial_cmp(&box_b.min[axis]).unwrap()
}

fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}
