use super::{Aabb, HitRecord, Object};
use crate::core::Ray;

use std::ops::Range;

use rand::prelude::*;

pub struct FlipFace<O> {
    object: O,
}

impl<O> FlipFace<O> {
    pub fn new(object: O) -> Self {
        Self { object }
    }
}

impl<O: Object> Object for FlipFace<O> {
    fn hit(&self, ray: &Ray, t_range: Range<f64>, rng: &mut ThreadRng) -> Option<HitRecord> {
        if let Some(mut rec) = self.object.hit(ray, t_range, rng) {
            rec.front_face = !rec.front_face;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, t_range: Range<f64>) -> Aabb {
        self.object.bounding_box(t_range)
    }
}
