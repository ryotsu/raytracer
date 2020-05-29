use rayon::prelude::*;

use raytracer::core::{Point, Ray, Vector};
use raytracer::objects::{HittableList, Sphere};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384 as u16;
    let image_height = (image_width as f64 / aspect_ratio) as u16;

    println!("P3\n {} {}\n255", image_width, image_height);

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vector::new(viewport_width, 0.0, 0.0);
    let vertical = Vector::new(0.0, viewport_height, 0.0);

    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vector::new(0.0, 0.0, focal_length);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {:>3}", j);

        let line = (0..image_width)
            .into_par_iter()
            .map(|i| {
                let u = i as f64 / (image_width as f64 - 1.0);
                let v = j as f64 / (image_height as f64 - 1.0);
                let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

                let pixel_color = ray.color(&world);
                pixel_color.write_color()
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
