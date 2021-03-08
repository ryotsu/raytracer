use super::{Aabb, HitRecord, Object};
use crate::core::{Point, Ray, Vector};
use crate::materials::Material;

use std::ops::Range;

use rand::prelude::*;

pub struct XZRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Material,
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Material) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Object for XZRect {
    fn hit(&self, ray: &Ray, t_range: Range<f64>, _rng: &mut ThreadRng) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y()) / ray.direction.y();
        if t < t_range.start || t > t_range.end {
            return None;
        }

        let x = ray.origin.x() + t * ray.direction.x();
        let z = ray.origin.z() + t * ray.direction.z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vector::new(0, 1, 0);
        let material = self.material.clone();
        let p = ray.at(t);

        let mut hit_rec = HitRecord {
            t,
            u,
            v,
            p,
            normal: outward_normal,
            material,
            front_face: true,
        };

        hit_rec.set_face_normal(ray, outward_normal);

        Some(hit_rec)
    }

    fn bounding_box(&self, _t_range: Range<f64>) -> Aabb {
        Aabb::new(
            Point::new(self.x0, self.k - 0.0001, self.z0),
            Point::new(self.x1, self.k + 0.0001, self.z1),
        )
    }
}
