use super::{Aabb, HitRecord, Object};
use crate::core::{Ray, Vector};

use std::sync::Arc;

use rand::prelude::*;

pub struct Translate {
    object: Arc<dyn Object>,
    offset: Vector,
}

impl Translate {
    pub fn new(object: Arc<dyn Object>, offset: Vector) -> Self {
        Self { object, offset }
    }
}

impl Object for Translate {
    fn hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
        rng: &mut ThreadRng,
    ) -> bool {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        if !self.object.hit(&moved_ray, t_min, t_max, rec, rng) {
            return false;
        }

        rec.p += self.offset;
        rec.set_face_normal(&moved_ray, rec.normal);

        true
    }

    fn bounding_box(&self, t_min: f64, t_max: f64) -> Aabb {
        let output_box = self.object.bounding_box(t_min, t_max);

        Aabb::new(output_box.min + self.offset, output_box.max + self.offset)
    }
}
