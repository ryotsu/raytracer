use super::{Aabb, HitRecord, Object};
use crate::core::{Point, Ray, Vector};
use crate::materials::Material;
use std::f64::consts::PI;

use rand::prelude::*;

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
    fn hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
        _rng: &mut ThreadRng,
    ) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root) / a;
            if t_min < temp && temp < t_max {
                rec.t = temp;
                rec.p = ray.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(ray, outward_normal);
                Self::get_sphere_uv((rec.p - self.center) / self.radius, &mut rec.u, &mut rec.v);
                rec.material = self.material.clone();
                return true;
            }
            temp = (-half_b + root) / a;
            if t_min < temp && temp < t_max {
                rec.t = temp;
                rec.p = ray.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(ray, outward_normal);
                Self::get_sphere_uv((rec.p - self.center) / self.radius, &mut rec.u, &mut rec.v);
                rec.material = self.material.clone();
                return true;
            }
        }

        false
    }

    fn bounding_box(&self, _t_min: f64, _t_max: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            self.center - Vector::from(self.radius),
            self.center + Vector::from(self.radius),
        );

        true
    }
}
