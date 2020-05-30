use super::{Aabb, HitRecord, Hittable};
use crate::core::{Point, Ray, Vector};
use crate::materials::Material;

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Box<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
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
                rec.material = self.material.box_clone();
                return true;
            }
            temp = (-half_b + root) / a;
            if t_min < temp && temp < t_max {
                rec.t = temp;
                rec.p = ray.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(ray, outward_normal);
                rec.material = self.material.box_clone();
                return true;
            }
        }

        false
    }

    fn bounding_box(&self, _t_min: f64, _t_max: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(
            self.center - Vector::new(self.radius, self.radius, self.radius),
            self.center + Vector::new(self.radius, self.radius, self.radius),
        );

        true
    }
}
