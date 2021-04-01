use crate::core::Point;
use crate::materials::Material::*;
use crate::objects::*;
use crate::textures::Noise;

use rand::prelude::*;

#[allow(dead_code)]
pub fn scene(rng: &mut ThreadRng) -> ObjectList {
    let mut world = ObjectList::new();

    let pertext = Noise::new(5.0, rng);

    world.add(Box::new(Sphere::new(
        Point::new(0, -1000, 0),
        1000.0,
        Lambertian {
            albedo: pertext.clone(),
        },
    )));

    world.add(Box::new(Sphere::new(
        Point::new(0, 2, 0),
        2.0,
        Lambertian { albedo: pertext },
    )));

    world
}
