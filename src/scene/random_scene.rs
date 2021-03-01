use crate::core::{Color, Point, Vector};
use crate::materials::{Dielectric, Lambertian, Material, Metal};
use crate::objects::*;
use crate::textures::{Checker, SolidColor};
use crate::utils::{random, random_in};

use std::sync::Arc;

#[allow(dead_code)]
pub fn scene() -> HittableList {
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
