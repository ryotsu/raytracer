use super::{Aabb, HitRecord, Hittable};
use crate::core::{Point, Ray};

use std::sync::Arc;

use rand::prelude::*;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
        rng: &mut ThreadRng,
    ) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec, rng) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.box_clone();
            }
        }

        return hit_anything;
    }

    fn bounding_box(&self, t_min: f64, t_max: f64, output_box: &mut Aabb) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box = Aabb::new(Point::from(0), Point::from(0));
        let mut first_box = true;

        for object in self.objects.iter() {
            if !object.bounding_box(t_min, t_max, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box.clone()
            } else {
                Aabb::surrounding_box(output_box, &temp_box)
            };

            first_box = false;
        }

        true
    }
}
