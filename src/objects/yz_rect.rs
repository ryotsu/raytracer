use super::{Aabb, HitRecord, Object};
use crate::core::{Point, Ray, Vector};
use crate::materials::Material;

use std::ops::Range;

use rand::prelude::*;

pub struct YZRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Material,
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Material) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Object for YZRect {
    fn hit(&self, ray: &Ray, t_range: Range<f64>, _rng: &mut ThreadRng) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x()) / ray.direction.x();
        if t < t_range.start || t > t_range.end {
            return None;
        }

        let y = ray.origin.y() + t * ray.direction.y();
        let z = ray.origin.z() + t * ray.direction.z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let t = t;
        let outward_normal = Vector::new(1, 0, 0);
        let material = &self.material;
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
            Point::new(self.k - 0.0001, self.y0, self.z0),
            Point::new(self.k + 0.0001, self.y1, self.z1),
        )
    }
}
