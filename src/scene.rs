use crate::core::{Color, Point, Vector};
use crate::materials::{Dielectric, Lambertian, Material, Metal};
use crate::objects::{HittableList, Sphere};
use crate::utils::{random, random_in};

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
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
                    let albedo = Color::random() * Color::random();
                    sphere_material = Box::new(Lambertian::new(albedo));
                } else if choose_material < 0.95 {
                    let albedo = Color::random_in(0.5, 1.0);
                    let fuzz = random_in(0.0, 0.5);
                    sphere_material = Box::new(Metal::new(albedo, fuzz));
                } else {
                    sphere_material = Box::new(Dielectric::new(1.5));
                }

                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Box::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}
