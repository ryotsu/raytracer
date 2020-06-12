use super::Texture;
use crate::core::{Color, Point};

#[derive(Clone)]
pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new<T: Into<f64>, U: Into<f64>, V: Into<f64>>(red: T, green: U, blue: V) -> Texture {
        Texture::SolidColor(Self {
            color: Color::new(red, green, blue),
        })
    }

    pub fn from<T: Into<f64> + Copy>(a: T) -> Texture {
        Self::new(a, a, a)
    }

    pub fn from_color(color: Color) -> Texture {
        Texture::SolidColor(Self { color })
    }

    pub fn value(&self, _u: f64, _v: f64, _p: Point) -> Color {
        self.color
    }
}
