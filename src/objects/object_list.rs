use super::{Aabb, HitRecord, Object};
use crate::core::{Point, Ray};

use std::ops::Range;

use rand::prelude::*;

pub struct ObjectList {
    pub objects: Vec<Box<dyn Object>>,
}

impl ObjectList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }
}

impl Object for ObjectList {
    fn hit(&self, ray: &Ray, t_range: Range<f64>, rng: &mut ThreadRng) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_range.end;

        for object in self.objects.iter() {
            if let Some(temp_rec) = object.hit(ray, t_range.start..closest_so_far, rng) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }

        return rec;
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
