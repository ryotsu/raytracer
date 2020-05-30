use rayon::prelude::*;

use raytracer::core::{Camera, Color, Point, Vector};
use raytracer::scene;
use raytracer::utils::random;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384 as u16;
    let image_height = (image_width as f64 / aspect_ratio) as u16;
    let samples_per_pixel = 100;
    let max_depth = 50;

    println!("P3\n {} {}\n255", image_width, image_height);

    let world = scene::random_scene();

    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let vup = Vector::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
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
