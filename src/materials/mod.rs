mod dielectric;
mod diffuse_light;
mod isotropic;
mod lambertian;
mod metal;

use crate::core::{Color, Point, Ray};
use crate::objects::HitRecord;

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use isotropic::Isotropic;
pub use lambertian::Lambertian;
pub use metal::Metal;

use rand::prelude::*;

#[derive(Clone)]
pub enum Material {
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
    Isotropic(Isotropic),
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material {
    pub fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Color, Ray)> {
        match self {
            Material::Dielectric(m) => m.scatter(ray_in, rec, rng),
            Material::DiffuseLight(m) => m.scatter(ray_in, rec),
            Material::Isotropic(m) => m.scatter(ray_in, rec, rng),
            Material::Lambertian(m) => m.scatter(ray_in, rec, rng),
            Material::Metal(m) => m.scatter(ray_in, rec, rng),
        }
    }

    pub fn emitted(&self, u: f64, v: f64, p: Point) -> Color {
        match self {
            Material::DiffuseLight(m) => m.emitted(u, v, p),
            _ => Color::from(0),
        }
    }
}
