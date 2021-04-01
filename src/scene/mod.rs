use crate::objects::ObjectList;

mod cornell_box;
mod cornell_smoke;
mod earth;
mod final_scene;
mod random_scene;
mod simple_light;
mod two_spheres;

use rand::prelude::*;

pub fn scene(rng: &mut ThreadRng) -> ObjectList {
    final_scene::scene(rng)
}
