use crate::core::{Color, Point, Vector};
use crate::materials::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::objects::{
    Boxx, ConstantMedium, FlipFace, HittableList, MovingSphere, RotateY, Sphere, Translate, XYRect,
    XZRect, YZRect,
};
use crate::textures::{Checker, Image, Noise, SolidColor};
use crate::utils::{random, random_in};

use std::sync::Arc;

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = Arc::new(Checker::new(
        Arc::new(SolidColor::from(0.2, 0.3, 0.1)),
        Arc::new(SolidColor::from(0.9, 0.9, 0.9)),
    ));

    let ground_material = Box::new(Lambertian::new(checker));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random();
            let center = Point::new(a as f64 + 0.9 * random(), 0.2, b as f64 + 0.9 * random());

            if (center - Vector::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Box<dyn Material>;

                if choose_material < 0.8 {
                    let albedo = SolidColor::new(Color::random() * Color::random());
                    sphere_material = Box::new(Lambertian::new(Arc::new(albedo)));
                    let center_max = center + Vector::new(0.0, random_in(0.0, 0.5), 0.0);
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
                    sphere_material = Box::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    sphere_material = Box::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Box::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Box::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.4, 0.2, 0.1,
    )))));
    world.add(Arc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

pub fn two_spheres() -> HittableList {
    let mut world = HittableList::new();

    let pertext = Arc::new(Noise::new(5.0));

    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(pertext.clone())),
    )));

    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        Box::new(Lambertian::new(pertext)),
    )));

    world
}

pub fn earth() -> HittableList {
    let mut world = HittableList::new();
    let earth_texture = Arc::new(Image::new("earthmap.jpg").unwrap());
    let earth_surace = Box::new(Lambertian::new(earth_texture));
    let globe = Arc::new(Sphere::new(Point::new(0.0, 0.0, 0.0), 2.0, earth_surace));

    world.add(globe);

    world
}

pub fn simple_light() -> HittableList {
    let mut world = HittableList::new();

    let pertext = Arc::new(Noise::new(4.0));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        Box::new(Lambertian::new(pertext)),
    )));

    let diffuse_light = DiffuseLight::new(Arc::new(SolidColor::new(Color::new(4.0, 4.0, 4.0))));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 7.0, 0.0),
        2.0,
        diffuse_light.box_clone(),
    )));
    world.add(Arc::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        Box::new(diffuse_light),
    )));

    world
}

pub fn cornell_box() -> HittableList {
    let mut world = HittableList::new();

    let red = Box::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.65, 0.05, 0.05,
    )))));
    let white = Box::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.73, 0.73, 0.73,
    )))));
    let green = Box::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.12, 0.45, 0.15,
    )))));
    let light = Box::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(
        15.0, 15.0, 15.0,
    )))));

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
        white.box_clone(),
    )))));
    world.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.box_clone(),
    )));
    world.add(Arc::new(FlipFace::new(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.box_clone(),
    )))));

    let box1 = Arc::new(Boxx::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 330.0, 165.0),
        white.box_clone(),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vector::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let box2 = Arc::new(Boxx::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 165.0, 165.0),
        white.box_clone(),
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vector::new(130.0, 0.0, 65.0)));
    world.add(box2);

    world
}

pub fn cornell_smoke() -> HittableList {
    let mut world = HittableList::new();

    let red = Box::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.65, 0.05, 0.05,
    )))));
    let white = Box::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.73, 0.73, 0.73,
    )))));
    let green = Box::new(Lambertian::new(Arc::new(SolidColor::new(Color::new(
        0.12, 0.45, 0.15,
    )))));
    let light = Box::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::new(
        7.0, 7.0, 7.0,
    )))));

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
        white.box_clone(),
    )))));
    world.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.box_clone(),
    )));
    world.add(Arc::new(FlipFace::new(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.box_clone(),
    )))));

    let box1 = Arc::new(Boxx::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 330.0, 165.0),
        white.box_clone(),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vector::new(265.0, 0.0, 295.0)));

    let box2 = Arc::new(Boxx::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 165.0, 165.0),
        white.box_clone(),
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vector::new(130.0, 0.0, 65.0)));

    world.add(Arc::new(ConstantMedium::new(
        box1,
        0.01,
        Arc::new(SolidColor::new(Color::new(0.0, 0.0, 0.0))),
    )));

    world.add(Arc::new(ConstantMedium::new(
        box2,
        0.01,
        Arc::new(SolidColor::new(Color::new(1.0, 1.0, 1.0))),
    )));

    world
}
