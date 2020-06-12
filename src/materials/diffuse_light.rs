use super::Material;
use crate::core::{Color, Point, Ray};
use crate::objects::HitRecord;
use crate::textures::Texture;

use std::sync::Arc;

#[derive(Clone)]
pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Material {
        Material::DiffuseLight(Self { emit })
    }

    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Point) -> Color {
        self.emit.value(u, v, p)
    }
}
