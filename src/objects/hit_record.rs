use crate::core::{Point, Ray, Vector};
use crate::materials::{Lambertian, Material};
use crate::textures::SolidColor;
use std::mem;

pub struct HitRecord {
    pub p: Point,
    pub normal: Vector,
    pub material: Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point::from(0),
            normal: Vector::from(0),
            material: Lambertian::new(SolidColor::from(0)),
            t: 0.0,
            u: 0.0,
            v: 0.0,
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
