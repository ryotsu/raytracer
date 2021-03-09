use crate::core::{Point, Ray, Vector};
use crate::materials::Material;

pub struct HitRecord<'m> {
    pub p: Point,
    pub normal: Vector,
    pub material: &'m Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl<'m> HitRecord<'m> {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
