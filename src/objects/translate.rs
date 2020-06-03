use super::{Aabb, HitRecord, Hittable};
use crate::core::{Ray, Vector};

use std::sync::Arc;

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vector,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vector) -> Self {
        Self { object, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        if !self.object.hit(&moved_ray, t_min, t_max, rec) {
            return false;
        }

        rec.p += self.offset;
        rec.set_face_normal(&moved_ray, rec.normal);

        true
    }
    fn bounding_box(&self, t_min: f64, t_max: f64, output_box: &mut Aabb) -> bool {
        if !self.object.bounding_box(t_min, t_max, output_box) {
            return false;
        }

        *output_box = Aabb::new(output_box.min + self.offset, output_box.max + self.offset);

        true
    }
}