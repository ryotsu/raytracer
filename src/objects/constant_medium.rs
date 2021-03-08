use super::{Aabb, HitRecord, Object};
use crate::core::{Ray, Vector};
use crate::materials::{Isotropic, Material};
use crate::textures::Texture;

use std::f64::{INFINITY, NEG_INFINITY};
use std::ops::Range;

use rand::prelude::*;

pub struct ConstantMedium<O> {
    boundary: O,
    phase_function: Material,
    neg_inv_density: f64,
}

impl<O> ConstantMedium<O> {
    pub fn new(boundary: O, density: f64, texture: Texture) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Isotropic::new(texture),
        }
    }
}

impl<O: Object> Object for ConstantMedium<O> {
    fn hit(&self, ray: &Ray, t_range: Range<f64>, rng: &mut ThreadRng) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(ray, NEG_INFINITY..INFINITY, rng) {
            if let Some(mut rec2) = self.boundary.hit(ray, (rec1.t + 0.0001)..INFINITY, rng) {
                if rec1.t < t_range.start {
                    rec1.t = t_range.start;
                }
                if rec2.t > t_range.end {
                    rec2.t = t_range.end;
                }

                if rec1.t >= rec2.t {
                    return None;
                }

                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }

                let ray_length = ray.direction.length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * rng.gen::<f64>().log2();

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = rec1.t + hit_distance / ray_length;
                let p = ray.at(t);
                let normal = Vector::new(1, 0, 0);
                let front_face = true;
                let material = self.phase_function.clone();

                return Some(HitRecord {
                    t,
                    u: 0.0,
                    v: 0.0,
                    p,
                    normal,
                    front_face,
                    material,
                });
            }
        }

        None
    }

    fn bounding_box(&self, t_range: Range<f64>) -> Aabb {
        self.boundary.bounding_box(t_range)
    }
}
