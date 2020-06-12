use crate::core::{Color, Point, Vector};
use crate::materials::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::objects::*;
use crate::textures::{Checker, Image, Noise, SolidColor};
use crate::utils::{random, random_in};

use std::sync::Arc;

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = Checker::new(
        Box::new(SolidColor::new(0.2, 0.3, 0.1)),
        Box::new(SolidColor::from(0.9)),
    );

    let ground_material = Lambertian::new(checker);
    world.add(Arc::new(Sphere::new(
        Point::new(0, -1000, 0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random();
            let center = Point::new(a as f64 + 0.9 * random(), 0.2, b as f64 + 0.9 * random());

            if (center - Vector::new(4, 0.2, 0)).length() > 0.9 {
                let sphere_material: Material;

                if choose_material < 0.8 {
                    let albedo = SolidColor::from_color(Color::random() * Color::random());
                    sphere_material = Lambertian::new(albedo);
                    let center_max = center + Vector::new(0, random_in(0.0, 0.5), 0);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center_max,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_material < 0.95 {
                    let albedo = Color::random_in(0.5, 1.0);
                    let fuzz = random_in(0.0, 0.5);
                    sphere_material = Metal::new(albedo, fuzz);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    sphere_material = Dielectric::new(1.5);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Arc::new(Sphere::new(Point::new(0, 1, 0), 1.0, material1)));

    let material2 = Lambertian::new(SolidColor::new(0.4, 0.2, 0.1));
    world.add(Arc::new(Sphere::new(Point::new(-4, 1, 0), 1.0, material2)));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Arc::new(Sphere::new(Point::new(4, 1, 0), 1.0, material3)));

    world
}

pub fn two_spheres() -> HittableList {
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

pub fn earth() -> HittableList {
    let mut world = HittableList::new();
    let earth_texture = Image::new("earthmap.jpg").unwrap();
    let earth_surace = Lambertian::new(earth_texture);
    let globe = Arc::new(Sphere::new(Point::from(0), 2.0, earth_surace));

    world.add(globe);

    world
}

pub fn simple_light() -> HittableList {
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

pub fn cornell_box() -> HittableList {
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

pub fn cornell_smoke() -> HittableList {
    let mut world = HittableList::new();

    let red = Lambertian::new(SolidColor::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(SolidColor::from(0.73));
    let green = Lambertian::new(SolidColor::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(SolidColor::from(7));

    world.add(Arc::new(FlipFace::new(Arc::new(YZRect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    )))));
    world.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add(Arc::new(XZRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    world.add(Arc::new(FlipFace::new(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )))));
    world.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
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

    let box2 = Arc::new(Boxx::new(Point::from(0), Point::from(165), white.clone()));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vector::new(130, 0, 65)));

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

pub fn final_scene() -> HittableList {
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
            let y1 = random_in(1.0, 101.0);
            let z1 = z0 + w;

            boxes.add(Arc::new(Boxx::new(
                Point::new(x0, y0, z0),
                Point::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut world = HittableList::new();

    world.add(Arc::new(BVHNode::new(&mut boxes.objects[..], 0.0, 1.0)));

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

    let boundary = Arc::new(Sphere::new(
        Point::new(360, 150, 145),
        70.0,
        Dielectric::new(1.5),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::new(
        boundary,
        0.2,
        SolidColor::new(0.2, 0.4, 0.9),
    )));

    let boundary = Arc::new(Sphere::new(Point::from(0), 5000.0, Dielectric::new(1.5)));
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

    let pertext = Lambertian::new(Noise::new(0.1));
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
            Point::random_in(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BVHNode::new(&mut boxes2.objects[..], 0.0, 1.0)),
            15.0,
        )),
        Vector::new(-100, 270, 395),
    )));

    world
}
