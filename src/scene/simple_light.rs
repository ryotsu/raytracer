use crate::core::Point;
use crate::materials::{DiffuseLight, Lambertian};
use crate::objects::*;
use crate::textures::{Noise, SolidColor};

use std::sync::Arc;

#[allow(dead_code)]
pub fn scene() -> HittableList {
    let mut world = HittableList::new();

    let pertext = Noise::new(4.0);
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

    let diffuse_light = DiffuseLight::new(SolidColor::from(4));
    world.add(Arc::new(Sphere::new(
        Point::new(0, 7, 0),
        2.0,
        diffuse_light.clone(),
    )));
    world.add(Arc::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        diffuse_light,
    )));

    world
}
