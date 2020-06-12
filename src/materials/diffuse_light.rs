use super::Material;
use crate::core::{Color, Point, Ray};
use crate::objects::HitRecord;
use crate::textures::Texture;

#[derive(Clone)]
pub struct DiffuseLight {
    emit: Texture,
}

impl DiffuseLight {
    pub fn new(emit: Texture) -> Material {
        Material::DiffuseLight(Self { emit })
    }

    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Point) -> Color {
        self.emit.value(u, v, p)
    }
}
