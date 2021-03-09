use crate::core::{Color, Point, Ray, Vector};
use crate::objects::HitRecord;
use crate::textures::Texture;
use crate::utils::schlick;

use rand::prelude::*;

#[derive(Clone)]
pub enum Material {
    Dielectric { ref_index: f64 },
    DiffuseLight { emit: Texture },
    Isotropic { albedo: Texture },
    Lambertian { albedo: Texture },
    Metal { albedo: Color, fuzz: f64 },
}

impl Material {
    pub fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Color, Ray)> {
        match self {
            Material::Dielectric { ref_index } => {
                let attenuation = Color::from(1);

                let etai_over_etat = if rec.front_face {
                    1.0 / ref_index
                } else {
                    *ref_index
                };

                let unit_direction = ray_in.direction.unit_vector();
                let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                if etai_over_etat * sin_theta > 1.0 {
                    let reflected = unit_direction.reflect(rec.normal);
                    let scattered = Ray::new(rec.p, reflected, ray_in.time);
                    return Some((attenuation, scattered));
                }

                let reflect_prob = schlick(cos_theta, etai_over_etat);
                if rng.gen::<f64>() < reflect_prob {
                    let reflected = unit_direction.reflect(rec.normal);
                    let scattered = Ray::new(rec.p, reflected, ray_in.time);
                    return Some((attenuation, scattered));
                }

                let refracted = unit_direction.refract(rec.normal, etai_over_etat);
                let scattered = Ray::new(rec.p, refracted, ray_in.time);
                Some((attenuation, scattered))
            }
            Material::DiffuseLight { .. } => None,
            Material::Isotropic { albedo } => {
                let scattered = Ray::new(rec.p, Vector::random_in_unit_sphere(rng), ray_in.time);
                let attenuation = albedo.value(rec.u, rec.v, rec.p);
                Some((attenuation, scattered))
            }
            Material::Lambertian { albedo } => {
                let scatter_direction = rec.normal + Vector::random_unit_vector(rng);
                let scattered = Ray::new(rec.p, scatter_direction, ray_in.time);
                let attenuation = albedo.value(rec.u, rec.v, rec.p);
                Some((attenuation, scattered))
            }
            Material::Metal { albedo, fuzz } => {
                let fuzz = fuzz.min(1.0);
                let reflected = ray_in.direction.unit_vector().reflect(rec.normal);
                let scattered = Ray::new(
                    rec.p,
                    reflected + Vector::random_in_unit_sphere(rng) * fuzz,
                    ray_in.time,
                );

                if scattered.direction.dot(rec.normal) > 0.0 {
                    Some((*albedo, scattered))
                } else {
                    None
                }
            }
        }
    }

    pub fn emitted(&self, u: f64, v: f64, p: Point) -> Color {
        match self {
            Material::DiffuseLight { emit } => emit.value(u, v, p),
            _ => Color::from(0),
        }
    }
}
