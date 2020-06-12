use crate::core::{Point, Ray};

#[derive(Clone)]
pub struct Aabb {
    pub min: Point,
    pub max: Point,
}

impl Aabb {
    pub fn new(min: Point, max: Point) -> Self {
        Self { min, max }
    }

    pub fn surrounding_box(a: &Aabb, b: &Aabb) -> Aabb {
        let small = Point::new(
            a.min.x().min(b.min.x()),
            a.min.y().min(b.min.y()),
            a.min.z().min(b.min.z()),
        );

        let big = Point::new(
            a.max.x().max(b.max.x()),
            a.max.y().max(b.max.y()),
            a.max.z().max(b.max.z()),
        );

        Aabb::new(small, big)
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction[a];

            let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = t0.max(t_min);
            t_max = t1.min(t_max);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}
