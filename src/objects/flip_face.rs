use super::{Aabb, HitRecord, Object};
use crate::core::Ray;

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
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
        rng: &mut ThreadRng,
    ) -> bool {
        if !self.hittable.hit(ray, t_min, t_max, rec, rng) {
            return false;
        }

        rec.front_face = !rec.front_face;
        true
    }

    fn bounding_box(&self, t_min: f64, t_max: f64, output_box: &mut Aabb) -> bool {
        self.hittable.bounding_box(t_min, t_max, output_box)
    }
}
