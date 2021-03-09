use super::Texture;
use crate::core::{Color, Point};
use crate::utils::clamp;

use image::{self, DynamicImage, GenericImageView, ImageError, Pixel};

#[derive(Clone)]
pub struct Image {
    img: Box<DynamicImage>,
}

impl Image {
    pub fn new(filename: &str) -> Result<Texture, ImageError> {
        let img = image::open(filename)?;
        Ok(Texture::Image(Self { img: Box::new(img) }))
    }

    pub fn value(&self, mut u: f64, mut v: f64, _p: Point) -> Color {
        u = clamp(u, 0.0, 1.0);
        v = 1.0 - clamp(v, 0.0, 1.0);

        let (width, height) = self.img.dimensions();

        let mut i = (u * width as f64) as u32;
        let mut j = (v * height as f64) as u32;

        if i >= width {
            i = width - 1;
        }

        if j >= height {
            j = height - 1;
        }

        let color_scale = 1.0 / 255.0;
        let image::Rgb([r, g, b]) = self.img.get_pixel(i, j).to_rgb();
        Color::new(
            r as f64 * color_scale,
            g as f64 * color_scale,
            b as f64 * color_scale,
        )
    }
}
