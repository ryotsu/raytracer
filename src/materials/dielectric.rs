use super::Material;
use crate::core::{Color, Ray};
use crate::objects::HitRecord;
use crate::utils::{random, schlick};

pub struct Dielectric {
    ref_index: f64,
}

impl Dielectric {
    pub fn new(ref_index: f64) -> Self {
        Self { ref_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::from(1);
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_index
        } else {
            self.ref_index
        };

        let unit_direction = ray_in.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = unit_direction.reflect(rec.normal);
            *scattered = Ray::new(rec.p, reflected, ray_in.time);
            return true;
        }

        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if random() < reflect_prob {
            let reflected = unit_direction.reflect(rec.normal);
            *scattered = Ray::new(rec.p, reflected, ray_in.time);
            return true;
        }

        let refracted = unit_direction.refract(rec.normal, etai_over_etat);
        *scattered = Ray::new(rec.p, refracted, ray_in.time);
        return true;
    }

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(Self::new(self.ref_index))
    }
}
