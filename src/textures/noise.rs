use super::perlin::Perlin;
use super::Texture;
use crate::core::{Color, Point};

use std::sync::Arc;

#[derive(Clone)]
pub struct Noise {
    noise: Arc<Perlin>,
    scale: f64,
}

impl Noise {
    pub fn new(scale: f64) -> Texture {
        Texture::Noise(Self {
            noise: Arc::new(Perlin::new()),
            scale,
        })
    }

    pub fn value(&self, _u: f64, _v: f64, p: Point) -> Color {
        Color::from(1) * 0.5 * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}
