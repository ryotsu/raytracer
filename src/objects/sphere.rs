use super::{Aabb, HitRecord, Object};
use crate::core::{Point, Ray, Vector};
use crate::materials::Material;
use std::f64::consts::PI;

use std::ops::Range;

use rand::prelude::*;

#[derive(Clone)]
pub struct Sphere {
    center: Point,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    fn get_sphere_uv(p: Point, u: &mut f64, v: &mut f64) {
        let phi = p.z().atan2(p.x());
        let theta = p.y().asin();
        *u = 1.0 - (phi + PI) / (2.0 * PI);
        *v = (theta + PI / 2.0) / PI;
    }
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_range: Range<f64>, _rng: &mut ThreadRng) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
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
                let outward_normal = (p - self.center) / self.radius;
                let (mut u, mut v) = (0.0, 0.0);
                Self::get_sphere_uv((p - self.center) / self.radius, &mut u, &mut v);
                let material = &self.material;
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
                return Some(hit_rec);
            }
            temp = (-half_b + root) / a;
            if t_range.start < temp && temp < t_range.end {
                let t = temp;
                let p = ray.at(t);
                let outward_normal = (p - self.center) / self.radius;
                let (mut u, mut v) = (0.0, 0.0);
                Self::get_sphere_uv((p - self.center) / self.radius, &mut u, &mut v);
                let material = &self.material;
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
                return Some(hit_rec);
            }
        }

        None
    }

    fn bounding_box(&self, _t_range: Range<f64>) -> Aabb {
        Aabb::new(
            self.center - Vector::from(self.radius),
            self.center + Vector::from(self.radius),
        )
    }
}
