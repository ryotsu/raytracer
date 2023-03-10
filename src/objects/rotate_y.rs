use super::{Aabb, HitRecord, Object};
use crate::core::{Point, Ray, Vector};
use crate::utils::degrees_to_radians;

use std::f64::INFINITY;
use std::ops::Range;

use rand::prelude::*;

pub struct RotateY<O> {
    object: O,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl<O: Object> RotateY<O> {
    pub fn new(object: O, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = object.bounding_box(0.0..1.0);

        let mut min = Point::from(INFINITY);
        let mut max = Point::from(-INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;

                    let x = i * bbox.max.x() + (1.0 - i) * bbox.min.x();
                    let y = j * bbox.max.y() + (1.0 - j) * bbox.min.y();
                    let z = k * bbox.max.z() + (1.0 - k) * bbox.min.z();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vector::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        bbox = Aabb::new(min, max);

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl<O: Object> Object for RotateY<O> {
    fn hit(&self, ray: &Ray, t_range: Range<f64>, rng: &mut ThreadRng) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated_ray = Ray::new(origin, direction, ray.time);

        if let Some(mut rec) = self.object.hit(&rotated_ray, t_range, rng) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

            normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

            rec.p = p;
            rec.set_face_normal(&rotated_ray, normal);

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _t_range: Range<f64>) -> Aabb {
        self.bbox.clone()
    }
}
