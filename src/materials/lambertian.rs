use super::Material;
use crate::core::{Color, Ray, Vector};
use crate::objects::HitRecord;

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + Vector::random_unit_vector();
        *scattered = Ray::new(rec.p, scatter_direction, ray_in.time);
        *attenuation = self.albedo;
        return true;
    }

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
