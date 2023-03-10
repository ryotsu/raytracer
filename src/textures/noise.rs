use super::perlin::Perlin;
use super::Texture;
use crate::core::{Color, Point};

use rand::prelude::*;

#[derive(Clone)]
pub struct Noise {
    noise: Box<Perlin>,
    scale: f64,
}

impl Noise {
    pub fn new_texture(scale: f64, rng: &mut ThreadRng) -> Texture {
        Texture::Noise(Self {
            noise: Box::new(Perlin::new(rng)),
            scale,
        })
    }

    pub fn value(&self, _u: f64, _v: f64, p: Point) -> Color {
        Color::from(0.5)
            * (1.0 + (self.scale * p.x() + 5.0 * self.noise.turb(p * self.scale, 7)).sin())
    }
}
