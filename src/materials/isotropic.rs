use super::Material;
use crate::core::{Color, Ray, Vector};
use crate::objects::HitRecord;
use crate::textures::Texture;

#[derive(Clone)]
pub struct Isotropic {
    albedo: Texture,
}

impl Isotropic {
    pub fn new(albedo: Texture) -> Material {
        Material::Isotropic(Self { albedo })
    }

    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scattered = Ray::new(rec.p, Vector::random_in_unit_sphere(), ray_in.time);
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        Some((attenuation, scattered))
    }
}
