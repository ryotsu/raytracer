mod aabb;
mod bvh;
mod flip_face;
mod hit_record;
mod hittable_list;
mod moving_sphere;
mod sphere;
mod xy_rect;
mod xz_rect;
mod yz_rect;

use crate::core::Ray;

pub use aabb::Aabb;
pub use bvh::BVHNode;
pub use flip_face::FlipFace;
pub use hit_record::HitRecord;
pub use hittable_list::HittableList;
pub use moving_sphere::MovingSphere;
pub use sphere::Sphere;
pub use xy_rect::XYRect;
pub use xz_rect::XZRect;
pub use yz_rect::YZRect;

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, t_min: f64, t_max: f64, output_box: &mut Aabb) -> bool;
}
