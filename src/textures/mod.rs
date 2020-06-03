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

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point) -> Color;
}
