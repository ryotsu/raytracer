use super::{Aabb, HitRecord, Object};
use crate::core::{Ray, Vector};

use std::ops::Range;

use rand::prelude::*;

pub struct Translate<O> {
    object: O,
    offset: Vector,
}

impl<O> Translate<O> {
    pub fn new(object: O, offset: Vector) -> Self {
        Self { object, offset }
    }
}

impl<O: Object> Object for Translate<O> {
    fn hit(
        &self,
        ray: &Ray,
        t_range: Range<f64>,
        rec: &mut HitRecord,
        rng: &mut ThreadRng,
    ) -> bool {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        if !self.object.hit(&moved_ray, t_range, rec, rng) {
            return false;
        }

        rec.p += self.offset;
        rec.set_face_normal(&moved_ray, rec.normal);

        true
    }

    fn bounding_box(&self, t_range: Range<f64>) -> Aabb {
        let output_box = self.object.bounding_box(t_range);

        Aabb::new(output_box.min + self.offset, output_box.max + self.offset)
    }
}
