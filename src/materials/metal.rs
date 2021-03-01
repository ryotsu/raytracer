use super::Material;
use crate::core::{Color, Ray, Vector};
use crate::objects::HitRecord;

use rand::prelude::*;

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Material {
        Material::Metal(Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        })
    }

    pub fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Color, Ray)> {
        let reflected = ray_in.direction.unit_vector().reflect(rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + Vector::random_in_unit_sphere(rng) * self.fuzz,
            ray_in.time,
        );
        let attenuation = self.albedo;

        if scattered.direction.dot(rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
