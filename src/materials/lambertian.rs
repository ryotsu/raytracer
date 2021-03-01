use super::Material;
use crate::core::{Color, Ray, Vector};
use crate::objects::HitRecord;
use crate::textures::Texture;

use rand::prelude::*;

#[derive(Clone)]
pub struct Lambertian {
    albedo: Texture,
}

impl Lambertian {
    pub fn new(albedo: Texture) -> Material {
        Material::Lambertian(Self { albedo })
    }

    pub fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Color, Ray)> {
        let scatter_direction = rec.normal + Vector::random_unit_vector(rng);
        let scattered = Ray::new(rec.p, scatter_direction, ray_in.time);
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        Some((attenuation, scattered))
    }
}
