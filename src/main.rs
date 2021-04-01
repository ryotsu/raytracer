use rayon::prelude::*;

use rand::prelude::*;

use raytracer::core::{Camera, Color, Point, Vector};
use raytracer::objects::Bvh;
use raytracer::scene;
use std::f64::INFINITY;

fn main() {
    let background = Color::from(0);

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920 as u16;
    let image_height = (image_width as f64 / aspect_ratio) as u16;
    let samples_per_pixel = 100;
    let max_depth = 50;

    println!("P3\n{} {}\n255", image_width, image_height);

    let mut rng = rand::thread_rng();

    let world = scene::scene(&mut rng);
    let world_bvh = Bvh::new(world.objects, 0.001..INFINITY, &mut rng);

    let look_from = Point::new(478, 278, -600);
    let look_at = Point::new(278, 278, 0);
    let vup = Vector::new(0, 1, 0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        40.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {:>4}", j);

        let line = (0..image_width)
            .into_par_iter()
            .map(|i| {
                let mut rng = thread_rng();
                let mut pixel_color = Color::from(0);
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.0);
                    let v = (j as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.0);
                    let ray = camera.ray(u, v, &mut rng);
                    pixel_color += ray.color(background, &world_bvh, max_depth, &mut rng);
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
