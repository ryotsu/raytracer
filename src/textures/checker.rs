use super::Texture;
use crate::core::{Color, Point};
use std::sync::Arc;

pub struct Checker {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl Checker {
    pub fn new(t0: Arc<dyn Texture>, t1: Arc<dyn Texture>) -> Self {
        Self { even: t0, odd: t1 }
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, p: Point) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
