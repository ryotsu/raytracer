use rayon::prelude::*;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n {} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {:>3}", j);

        let line = (0..image_width)
            .into_par_iter()
            .map(|i| {
                let r = i as f64 / (image_width - 1) as f64;
                let g = j as f64 / (image_height - 1) as f64;
                let b = 0.25;

                let ir = (255.999 * r) as u64;
                let ig = (255.999 * g) as u64;
                let ib = (255.999 * b) as u64;

                format!("{} {} {}", ir, ig, ib)
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
