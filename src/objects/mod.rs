mod aabb;
mod boxx;
mod bvh;
mod constant_medium;
mod flip_face;
mod hit_record;
mod hittable_list;
mod moving_sphere;
mod rotate_y;
mod sphere;
mod translate;
mod xy_rect;
mod xz_rect;
mod yz_rect;

use crate::core::Ray;

pub use aabb::Aabb;
pub use boxx::Boxx;
pub use bvh::BVHNode;
pub use constant_medium::ConstantMedium;
pub use flip_face::FlipFace;
pub use hit_record::HitRecord;
pub use hittable_list::HittableList;
pub use moving_sphere::MovingSphere;
pub use rotate_y::RotateY;
pub use sphere::Sphere;
pub use translate::Translate;
pub use xy_rect::XYRect;
pub use xz_rect::XZRect;
pub use yz_rect::YZRect;

use rand::prelude::*;

use std::ops::Range;

pub trait Object: Send + Sync {
    fn hit(&self, ray: &Ray, t_range: Range<f64>, rng: &mut ThreadRng) -> Option<HitRecord>;

    fn bounding_box(&self, t_range: Range<f64>) -> Aabb;
}
