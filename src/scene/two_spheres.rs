use crate::core::Point;
use crate::materials::Lambertian;
use crate::objects::*;
use crate::textures::Noise;

use std::sync::Arc;

#[allow(dead_code)]
pub fn scene() -> HittableList {
    let mut world = HittableList::new();

    let pertext = Noise::new(5.0);

    world.add(Arc::new(Sphere::new(
        Point::new(0, -1000, 0),
        1000.0,
        Lambertian::new(pertext.clone()),
    )));

    world.add(Arc::new(Sphere::new(
        Point::new(0, 2, 0),
        2.0,
        Lambertian::new(pertext),
    )));

    world
}
