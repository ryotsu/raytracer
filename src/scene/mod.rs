use crate::objects::HittableList;

mod cornell_box;
mod cornell_smoke;
mod earth;
mod final_scene;
mod random_scene;
mod simple_light;
mod two_spheres;

pub fn scene() -> HittableList {
    final_scene::scene()
}
