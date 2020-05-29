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

    pub fn color<T: Hittable>(&self, world: &T) -> Color {
        let mut rec = HitRecord::new();

        if world.hit(self, 0.0, INFINITY, &mut rec) {
            return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_direction = self.direction.unit_vector();
        let t = (unit_direction.y + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}
