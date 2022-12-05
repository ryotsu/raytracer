use crate::core::{Point, Vector};
use crate::materials::Material::*;
use crate::objects::*;
use crate::textures::SolidColor;

use rand::prelude::*;

#[allow(dead_code)]
pub fn scene(_rng: &mut ThreadRng) -> ObjectList {
    let mut world = ObjectList::new();

    let red = Lambertian {
        albedo: SolidColor::new_texture(0.65, 0.05, 0.05),
    };
    let white = Lambertian {
        albedo: SolidColor::from(0.73),
    };
    let green = Lambertian {
        albedo: SolidColor::new_texture(0.12, 0.45, 0.15),
    };
    let light = DiffuseLight {
        emit: SolidColor::from(15),
    };

    world.add(Box::new(FlipFace::new(YZRect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    ))));
    world.add(Box::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add(Box::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    world.add(Box::new(FlipFace::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    ))));
    world.add(Box::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Box::new(FlipFace::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    ))));

    let box1 = Boxx::new(Point::from(0), Point::new(165, 330, 165), white.clone());
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Box::new(Translate::new(box1, Vector::new(265, 0, 295)));
    world.add(box1);

    let box2 = Boxx::new(Point::from(0), Point::from(165), white);
    let box2 = RotateY::new(box2, -18.0);
    let box2 = Box::new(Translate::new(box2, Vector::new(130, 0, 65)));
    world.add(box2);

    world
}
