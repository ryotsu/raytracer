use rayon::prelude::*;

use raytracer::core::{Camera, Color, Point, Vector};
use raytracer::materials::{Dielectric, Lambertian, Metal};
use raytracer::objects::{HittableList, Sphere};
use raytracer::utils::random;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384 as u16;
    let image_height = (image_width as f64 / aspect_ratio) as u16;
    let samples_per_pixel = 100;
    let max_depth = 50;

    println!("P3\n {} {}\n255", image_width, image_height);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0)),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        -0.45,
        Box::new(Dielectric::new(1.5)),
    )));

    let camera = Camera::new(
        Point::new(-2.0, 2.0, 1.0),
        Point::new(0.0, 0.0, -1.0),
        Vector::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
    );

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {:>3}", j);

        let line = (0..image_width)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + random()) / (image_width as f64 - 1.0);
                    let v = (j as f64 + random()) / (image_height as f64 - 1.0);
                    let ray = camera.ray(u, v);
                    pixel_color += ray.color(&world, max_depth);
                }
                pixel_color.write_color(samples_per_pixel)
            })
            .reduce(
                || String::new(),
                |mut acc, val| {
                    acc.push_str(&val);
                    acc.push(' ');
                    acc
                },
            );

        println!("{}", line);
    }
    eprintln!("\nDone!");
}
