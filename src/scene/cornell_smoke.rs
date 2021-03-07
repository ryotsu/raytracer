use crate::core::{Point, Vector};
use crate::materials::{DiffuseLight, Lambertian};
use crate::objects::*;
use crate::textures::SolidColor;

use std::sync::Arc;

use rand::prelude::*;

#[allow(dead_code)]
pub fn scene(_rng: &mut ThreadRng) -> HittableList {
    let mut world = HittableList::new();

    let red = Lambertian::new(SolidColor::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(SolidColor::from(0.73));
    let green = Lambertian::new(SolidColor::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(SolidColor::from(7));

    world.add(Arc::new(FlipFace::new(YZRect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    ))));
    world.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add(Arc::new(XZRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    world.add(Arc::new(FlipFace::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    ))));
    world.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.add(Arc::new(FlipFace::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    ))));

    let box1 = Boxx::new(Point::from(0), Point::new(165, 330, 165), white.clone());
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Translate::new(box1, Vector::new(265, 0, 295));

    let box2 = Boxx::new(Point::from(0), Point::from(165), white.clone());
    let box2 = RotateY::new(box2, -18.0);
    let box2 = Translate::new(box2, Vector::new(130, 0, 65));

    world.add(Arc::new(ConstantMedium::new(
        box1,
        0.01,
        SolidColor::from(0),
    )));

    world.add(Arc::new(ConstantMedium::new(
        box2,
        0.01,
        SolidColor::from(1),
    )));

    world
}
