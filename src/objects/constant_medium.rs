use super::{Aabb, HitRecord, Object};
use crate::core::{Ray, Vector};
use crate::materials::{Isotropic, Material};
use crate::textures::Texture;

use std::f64::INFINITY;
use std::sync::Arc;

use rand::prelude::*;

pub struct ConstantMedium {
    boundary: Arc<dyn Object>,
    phase_function: Material,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Object>, density: f64, texture: Texture) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Isotropic::new(texture),
        }
    }
}

impl Object for ConstantMedium {
    fn hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
        rng: &mut ThreadRng,
    ) -> bool {
        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();

        if !self.boundary.hit(ray, -INFINITY, INFINITY, &mut rec1, rng) {
            return false;
        }

        if !self
            .boundary
            .hit(ray, rec1.t + 0.0001, INFINITY, &mut rec2, rng)
        {
            return false;
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rng.gen::<f64>().log2();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = ray.at(rec.t);

        rec.normal = Vector::new(1, 0, 0);
        rec.front_face = true;
        rec.material = self.phase_function.clone();

        true
    }

    fn bounding_box(&self, t_min: f64, t_max: f64, output_box: &mut Aabb) -> bool {
        self.boundary.bounding_box(t_min, t_max, output_box)
    }
}
