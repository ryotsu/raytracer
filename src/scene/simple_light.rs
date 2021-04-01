use crate::core::Point;
use crate::materials::Material::*;
use crate::objects::*;
use crate::textures::{Noise, SolidColor};

use rand::prelude::*;

#[allow(dead_code)]
pub fn scene(rng: &mut ThreadRng) -> HittableList {
    let mut world = HittableList::new();

    let pertext = Noise::new(4.0, rng);
    world.add(Box::new(Sphere::new(
        Point::new(0, -1000, 0),
        1000.0,
        Lambertian {
            albedo: pertext.clone(),
        },
    )));
    world.add(Box::new(Sphere::new(
        Point::new(0, 2, 0),
        2.0,
        Lambertian { albedo: pertext },
    )));

    let diffuse_light = DiffuseLight {
        emit: SolidColor::from(4),
    };
    world.add(Box::new(Sphere::new(
        Point::new(0, 7, 0),
        2.0,
        diffuse_light.clone(),
    )));
    world.add(Box::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        diffuse_light,
    )));

    world
}
