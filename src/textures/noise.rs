use super::perlin::Perlin;
use super::Texture;
use crate::core::{Color, Point};

use std::sync::Arc;

use rand::prelude::*;

#[derive(Clone)]
pub struct Noise {
    noise: Arc<Perlin>,
    scale: f64,
}

impl Noise {
    pub fn new(scale: f64, rng: &mut ThreadRng) -> Texture {
        Texture::Noise(Self {
            noise: Arc::new(Perlin::new(rng)),
            scale,
        })
    }

    pub fn value(&self, _u: f64, _v: f64, p: Point) -> Color {
        Color::from(0.5)
            * (1.0 + (self.scale * p.x() + 5.0 * self.noise.turb(p * self.scale, 7)).sin())
    }
}
