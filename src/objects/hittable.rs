use super::Aabb;
use crate::core::{Color, Point, Ray, Vector};
use crate::materials::{Lambertian, Material};

use std::mem;

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, t_min: f64, t_max: f64, output_box: &mut Aabb) -> bool;
}

pub struct HitRecord {
    pub p: Point,
    pub normal: Vector,
    pub material: Box<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point::new(0.0, 0.0, 0.0),
            normal: Vector::new(0.0, 0.0, 0.0),
            material: Box::new(Lambertian::new(Color::new(0.0, 0.0, 0.0))),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    pub fn box_clone(&mut self) -> Self {
        mem::replace(self, Self::new())
    }
}
