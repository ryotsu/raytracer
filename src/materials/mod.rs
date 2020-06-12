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

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;

    #[allow(unused_variables)]
    fn emitted(&self, u: f64, v: f64, p: Point) -> Color {
        Color::from(0)
    }

    fn box_clone(&self) -> Box<dyn Material>;
}
