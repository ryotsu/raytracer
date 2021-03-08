use super::{Aabb, HitRecord, Object};
use crate::core::{Point, Ray, Vector};
use crate::materials::Material;

use std::ops::Range;

use rand::prelude::*;

pub struct XYRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: Material,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Material) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl Object for XYRect {
    fn hit(&self, ray: &Ray, t_range: Range<f64>, _rng: &mut ThreadRng) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z()) / ray.direction.z();
        if t < t_range.start || t > t_range.end {
            return None;
        }

        let x = ray.origin.x() + t * ray.direction.x();
        let y = ray.origin.y() + t * ray.direction.y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = Vector::new(0, 0, 1);
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
            Point::new(self.x0, self.y0, self.k - 0.0001),
            Point::new(self.x1, self.y1, self.k + 0.0001),
        )
    }
}
