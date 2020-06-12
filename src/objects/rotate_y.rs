use super::{Aabb, HitRecord, Hittable};
use crate::core::{Point, Ray, Vector};
use crate::utils::degrees_to_radians;

use std::f64::INFINITY;
use std::sync::Arc;

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = Aabb::new(Point::from(0), Point::from(0));
        let hasbox = object.bounding_box(0.0, 1.0, &mut bbox);

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
            hasbox,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin[0] = self.cos_theta * ray.origin[0] - self.sin_theta * ray.origin[2];
        origin[2] = self.sin_theta * ray.origin[0] + self.cos_theta * ray.origin[2];

        direction[0] = self.cos_theta * ray.direction[0] - self.sin_theta * ray.direction[2];
        direction[2] = self.sin_theta * ray.direction[0] + self.cos_theta * ray.direction[2];

        let rotated_ray = Ray::new(origin, direction, ray.time);

        if !self.object.hit(&rotated_ray, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p;
        let mut normal = rec.normal;

        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.set_face_normal(&rotated_ray, normal);

        true
    }

    fn bounding_box(&self, _t_min: f64, _t_max: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox.clone();
        self.hasbox
    }
}
