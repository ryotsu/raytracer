use super::Material;
use crate::core::{Color, Ray, Vector};
use crate::objects::HitRecord;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = ray_in.direction.unit_vector().reflect(rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + Vector::random_in_unit_sphere() * self.fuzz,
            ray_in.time,
        );
        *attenuation = self.albedo;
        scattered.direction.dot(rec.normal) > 0.0
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(Self::new(self.albedo, self.fuzz))
    }
}
