use super::{Aabb, HitRecord, Hittable};
use crate::core::{Ray, Vector};
use crate::materials::{Isotropic, Material};
use crate::textures::Texture;
use crate::utils::random;

use std::f64::INFINITY;
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Box<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, texture: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Box::new(Isotropic::new(texture)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let enable_debug = false;
        let debugging = enable_debug && random() < 0.00001;

        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();

        if !self.boundary.hit(ray, -INFINITY, INFINITY, &mut rec1) {
            return false;
        }

        if !self.boundary.hit(ray, rec1.t + 0.0001, INFINITY, &mut rec2) {
            return false;
        }

        if debugging {
            eprintln!("\nt0={}, t1={}", rec1.t, rec2.t);
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
        let hit_distance = self.neg_inv_density * random().log2();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = ray.at(rec.t);

        if debugging {
            eprintln!("hit_distance = {}", hit_distance);
            eprintln!("rec.t = {}", rec.t);
            eprintln!("rec.p = {}", rec.p);
        }

        rec.normal = Vector::new(1, 0, 0);
        rec.front_face = true;
        rec.material = self.phase_function.box_clone();

        true
    }

    fn bounding_box(&self, t_min: f64, t_max: f64, output_box: &mut Aabb) -> bool {
        self.boundary.bounding_box(t_min, t_max, output_box)
    }
}
