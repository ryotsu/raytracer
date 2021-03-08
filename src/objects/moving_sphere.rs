use super::{Aabb, HitRecord, Object};
use crate::core::{Point, Ray, Vector};
use crate::materials::Material;

use std::ops::Range;

use rand::prelude::*;

pub struct MovingSphere {
    center_min: Point,
    center_max: Point,
    time_min: f64,
    time_max: f64,
    radius: f64,
    material: Material,
}

impl MovingSphere {
    pub fn new(
        center_min: Point,
        center_max: Point,
        time_min: f64,
        time_max: f64,
        radius: f64,
        material: Material,
    ) -> Self {
        Self {
            center_min,
            center_max,
            time_min,
            time_max,
            radius,
            material,
        }
    }

    fn center(&self, time: f64) -> Point {
        self.center_min
            + (self.center_max - self.center_min) * (time - self.time_min)
                / (self.time_max - self.time_min)
    }
}

impl Object for MovingSphere {
    fn hit(&self, ray: &Ray, t_range: Range<f64>, _rng: &mut ThreadRng) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let mut temp = (-half_b - root) / a;
            if t_range.start < temp && temp < t_range.end {
                let t = temp;
                let p = ray.at(t);
                let outward_normal = (p - self.center(ray.time)) / self.radius;
                let material = self.material.clone();

                let mut hit_rec = HitRecord {
                    t,
                    u: 0.0,
                    v: 0.0,
                    p,
                    normal: outward_normal,
                    material,
                    front_face: true,
                };

                hit_rec.set_face_normal(ray, outward_normal);
                return Some(hit_rec);
            }

            temp = (-half_b + root) / a;
            if t_range.start < temp && temp < t_range.end {
                let t = temp;
                let p = ray.at(t);
                let outward_normal = (p - self.center(ray.time)) / self.radius;
                let material = self.material.clone();

                let mut hit_rec = HitRecord {
                    t,
                    u: 0.0,
                    v: 0.0,
                    p,
                    normal: outward_normal,
                    material,
                    front_face: true,
                };

                hit_rec.set_face_normal(ray, outward_normal);
                return Some(hit_rec);
            }
        }

        None
    }

    fn bounding_box(&self, t_range: Range<f64>) -> Aabb {
        let box0 = Aabb::new(
            self.center(t_range.start) - Vector::from(self.radius),
            self.center(t_range.start) + Vector::from(self.radius),
        );

        let box1 = Aabb::new(
            self.center(t_range.end) - Vector::from(self.radius),
            self.center(t_range.end) + Vector::from(self.radius),
        );

        Aabb::surrounding_box(&box0, &box1)
    }
}
