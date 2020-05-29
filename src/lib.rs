pub mod core;

pub fn hit_sphere(center: core::Point, radius: f64, ray: &core::Ray) -> bool {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = oc.dot(ray.direction) * 2.0;
    let c = oc.dot(oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}
