use super::Material;
use crate::core::{Color, Ray, Vector};
use crate::objects::HitRecord;
use crate::textures::Texture;

use std::sync::Arc;

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scattered = Ray::new(rec.p, Vector::random_in_unit_sphere(), ray_in.time);
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        Some((attenuation, scattered))
    }

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(Self::new(self.albedo.clone()))
    }
}
