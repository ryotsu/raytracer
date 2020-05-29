mod lambertian;
mod metal;

use crate::core::{Color, Ray};
use crate::objects::HitRecord;

pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    fn box_clone(&self) -> Box<dyn Material>;
}
