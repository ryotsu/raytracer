use super::{Color, Point, Vector};
use crate::objects::{HitRecord, Hittable};

use std::f64::INFINITY;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn color<T: Hittable>(&self, world: &T, depth: i8) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::new();

        if world.hit(self, 0.001, INFINITY, &mut rec) {
            let target = rec.p + Point::random_in_hemisphere(rec.normal);
            return Ray::new(rec.p, target - rec.p).color(world, depth - 1) * 0.5;
        }

        let unit_direction = self.direction.unit_vector();
        let t = (unit_direction.y + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}
