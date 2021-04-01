mod aabb;
mod boxx;
mod bvh;
mod constant_medium;
mod flip_face;
mod hit_record;
mod moving_sphere;
mod object_list;
mod rotate_y;
mod sphere;
mod translate;
mod xy_rect;
mod xz_rect;
mod yz_rect;

use crate::core::Ray;

pub use aabb::Aabb;
pub use boxx::Boxx;
pub use bvh::Bvh;
pub use constant_medium::ConstantMedium;
pub use flip_face::FlipFace;
pub use hit_record::HitRecord;
pub use moving_sphere::MovingSphere;
pub use object_list::ObjectList;
pub use rotate_y::RotateY;
pub use sphere::Sphere;
pub use translate::Translate;
pub use xy_rect::XYRect;
pub use xz_rect::XZRect;
pub use yz_rect::YZRect;

use rand::prelude::*;

use std::ops::Range;

pub trait Object: Send + Sync {
    fn hit<'o>(
        &'o self,
        ray: &Ray,
        t_range: Range<f64>,
        rng: &mut ThreadRng,
    ) -> Option<HitRecord<'o>>;

    fn bounding_box(&self, t_range: Range<f64>) -> Aabb;
}
