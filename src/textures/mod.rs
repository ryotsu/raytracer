mod checker;
mod image;
mod noise;
mod perlin;
mod solid_color;

use crate::core::{Color, Point};

pub use self::image::Image;
pub use checker::Checker;
pub use noise::Noise;
pub use solid_color::SolidColor;

#[derive(Clone)]
pub enum Texture {
    Checker(Checker),
    Image(Image),
    Noise(Noise),
    SolidColor(SolidColor),
}

impl Texture {
    pub fn value(&self, u: f64, v: f64, p: Point) -> Color {
        match self {
            Texture::Checker(t) => t.value(u, v, p),
            Texture::Image(t) => t.value(u, v, p),
            Texture::Noise(t) => t.value(u, v, p),
            Texture::SolidColor(t) => t.value(u, v, p),
        }
    }
}
