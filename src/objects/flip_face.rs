use super::{Aabb, HitRecord, Object};
use crate::core::Ray;

use std::ops::Range;
use std::sync::Arc;

use rand::prelude::*;

pub struct FlipFace {
    hittable: Arc<dyn Object>,
}

impl FlipFace {
    pub fn new(hittable: Arc<dyn Object>) -> Self {
        Self { hittable }
    }
}

impl Object for FlipFace {
    fn hit(
        &self,
        ray: &Ray,
        t_range: Range<f64>,
        rec: &mut HitRecord,
        rng: &mut ThreadRng,
    ) -> bool {
        if !self.hittable.hit(ray, t_range, rec, rng) {
            return false;
        }

        rec.front_face = !rec.front_face;
        true
    }

    fn bounding_box(&self, t_range: Range<f64>) -> Aabb {
        self.hittable.bounding_box(t_range)
    }
}
