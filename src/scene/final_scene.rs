use crate::core::{Color, Point, Vector};
use crate::materials::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::objects::*;
use crate::textures::{Image, Noise, SolidColor};

use std::sync::Arc;

use rand::prelude::*;

#[allow(dead_code)]
pub fn scene(rng: &mut ThreadRng) -> HittableList {
    let mut boxes = HittableList::new();

    let ground = Lambertian::new(SolidColor::new(0.48, 0.93, 0.53));

    let boxes_per_side = 20;

    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes.add(Arc::new(Boxx::new(
                Point::new(x0, y0, z0),
                Point::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut world = HittableList::new();

    world.add(Arc::new(BVHNode::new(
        &mut boxes.objects[..],
        0.0,
        1.0,
        rng,
    )));

    let light = DiffuseLight::new(SolidColor::from(7.0));
    world.add(Arc::new(XZRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let center1 = Point::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vector::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian::new(SolidColor::new(0.7, 0.3, 0.1));
    world.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    world.add(Arc::new(Sphere::new(
        Point::new(260, 150, 45),
        50.0,
        Dielectric::new(1.5),
    )));

    world.add(Arc::new(Sphere::new(
        Point::new(0, 150, 145),
        50.0,
        Metal::new(Color::new(0.8, 0.8, 0.9), 10.0),
    )));

    let boundary = Sphere::new(Point::new(360, 150, 145), 70.0, Dielectric::new(1.5));
    world.add(Arc::new(boundary.clone()));
    world.add(Arc::new(ConstantMedium::new(
        boundary,
        0.2,
        SolidColor::new(0.2, 0.4, 0.9),
    )));

    let boundary = Sphere::new(Point::from(0), 5000.0, Dielectric::new(1.5));
    world.add(Arc::new(ConstantMedium::new(
        boundary,
        0.0001,
        SolidColor::from(1),
    )));

    let emat = Lambertian::new(Image::new("earthmap.jpg").unwrap());
    world.add(Arc::new(Sphere::new(
        Point::new(400, 200, 400),
        100.0,
        emat,
    )));

    let pertext = Lambertian::new(Noise::new(0.05, rng));
    world.add(Arc::new(Sphere::new(
        Point::new(220, 280, 300),
        80.0,
        pertext,
    )));

    let mut boxes2 = HittableList::new();
    let white = Lambertian::new(SolidColor::from(0.73));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Point::random_in(0.0, 165.0, rng),
            10.0,
            white.clone(),
        )));
    }

    world.add(Arc::new(Translate::new(
        RotateY::new(BVHNode::new(&mut boxes2.objects[..], 0.0, 1.0, rng), 15.0),
        Vector::new(-100, 270, 395),
    )));

    world
}
