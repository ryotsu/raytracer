use super::{Aabb, HitRecord, Hittable};
use crate::core::{Point, Ray, Vector};
use crate::materials::Material;

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

impl Hittable for MovingSphere {
    fn hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
        _rng: &mut ThreadRng,
    ) -> bool {
        let oc = ray.origin - self.center(ray.time);
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
                let outward_normal = (rec.p - self.center(ray.time)) / self.radius;
                rec.set_face_normal(ray, outward_normal);
                rec.material = self.material.clone();
                return true;
            }

            temp = (-half_b + root) / a;
            if t_min < temp && temp < t_max {
                rec.t = temp;
                rec.p = ray.at(rec.t);
                let outward_normal = (rec.p - self.center(ray.time)) / self.radius;
                rec.set_face_normal(ray, outward_normal);
                rec.material = self.material.clone();
                return true;
            }
        }

        return false;
    }
    fn bounding_box(&self, t_min: f64, t_max: f64, output_box: &mut Aabb) -> bool {
        let box0 = Aabb::new(
            self.center(t_min) - Vector::from(self.radius),
            self.center(t_min) + Vector::from(self.radius),
        );

        let box1 = Aabb::new(
            self.center(t_max) - Vector::from(self.radius),
            self.center(t_max) + Vector::from(self.radius),
        );

        *output_box = Aabb::surrounding_box(&box0, &box1);

        true
    }
}
