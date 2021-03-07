use super::{Aabb, HitRecord, Object};
use crate::core::{Point, Ray};

use std::ops::Range;
use std::sync::Arc;

use rand::prelude::*;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Object>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Object>) {
        self.objects.push(object);
    }
}

impl Object for HittableList {
    fn hit(
        &self,
        ray: &Ray,
        t_range: Range<f64>,
        rec: &mut HitRecord,
        rng: &mut ThreadRng,
    ) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_range.end;

        for object in self.objects.iter() {
            if object.hit(ray, t_range.start..closest_so_far, &mut temp_rec, rng) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.box_clone();
            }
        }

        return hit_anything;
    }

    fn bounding_box(&self, t_range: Range<f64>) -> Aabb {
        self.objects
            .iter()
            .fold(Aabb::new(Point::from(0), Point::from(0)), |acc, x| {
                let bounding_box = x.bounding_box(t_range.clone());
                Aabb::surrounding_box(&acc, &bounding_box)
            })
    }
}
