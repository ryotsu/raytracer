use crate::core::Point;
use crate::materials::Lambertian;
use crate::objects::*;
use crate::textures::Image;

use std::sync::Arc;

use rand::prelude::*;

#[allow(dead_code)]
pub fn scene(_rng: &mut ThreadRng) -> HittableList {
    let mut world = HittableList::new();
    let earth_texture = Image::new("earthmap.jpg").unwrap();
    let earth_surace = Lambertian::new(earth_texture);
    let globe = Arc::new(Sphere::new(Point::from(0), 2.0, earth_surace));

    world.add(globe);

    world
}
