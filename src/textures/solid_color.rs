use super::Texture;
use crate::core::{Color, Point};

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn from(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: Point) -> Color {
        self.color
    }
}
