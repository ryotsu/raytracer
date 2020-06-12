use super::Texture;
use crate::core::{Color, Point};

#[derive(Clone)]
pub struct Checker {
    odd: Box<Texture>,
    even: Box<Texture>,
}

impl Checker {
    pub fn new(even: Box<Texture>, odd: Box<Texture>) -> Texture {
        Texture::Checker(Self { even, odd })
    }

    pub fn value(&self, u: f64, v: f64, p: Point) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
