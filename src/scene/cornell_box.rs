use crate::core::{Point, Vector};
use crate::materials::{DiffuseLight, Lambertian};
use crate::objects::*;
use crate::textures::SolidColor;

use std::sync::Arc;

#[allow(dead_code)]
pub fn scene() -> HittableList {
    let mut world = HittableList::new();

    let red = Lambertian::new(SolidColor::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(SolidColor::from(0.73));
    let green = Lambertian::new(SolidColor::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(SolidColor::from(15));

    world.add(Arc::new(FlipFace::new(Arc::new(YZRect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    )))));
    world.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add(Arc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    world.add(Arc::new(FlipFace::new(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )))));
    world.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Arc::new(FlipFace::new(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )))));

    let box1 = Arc::new(Boxx::new(
        Point::from(0),
        Point::new(165, 330, 165),
        white.clone(),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vector::new(265, 0, 295)));
    world.add(box1);

    let box2 = Arc::new(Boxx::new(Point::from(0), Point::from(165), white.clone()));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vector::new(130, 0, 65)));
    world.add(box2);

    world
}
