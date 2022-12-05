use crate::core::Point;
use crate::materials::Material::*;
use crate::objects::*;
use crate::textures::Image;

use rand::prelude::*;

#[allow(dead_code)]
pub fn scene(_rng: &mut ThreadRng) -> ObjectList {
    let mut world = ObjectList::new();
    let earth_texture = Image::new_image("earthmap.jpg").unwrap();
    let earth_surace = Lambertian {
        albedo: earth_texture,
    };
    let globe = Box::new(Sphere::new(Point::from(0), 2.0, earth_surace));

    world.add(globe);

    world
}
