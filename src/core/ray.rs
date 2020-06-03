use super::{Color, Point, Vector};
use crate::objects::{HitRecord, Hittable};

use std::f64::INFINITY;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn color<T: Hittable>(&self, background: Color, world: &T, depth: i8) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::new();

        if !world.hit(self, 0.001, INFINITY, &mut rec) {
            return background;
        }

        let mut scattered = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0), 0.0);
        let mut attenuation = Color::new(0.0, 0.0, 0.0);
        let emitted = rec.material.emitted(rec.u, rec.v, rec.p);

        if !rec
            .material
            .scatter(self, &rec, &mut attenuation, &mut scattered)
        {
            return emitted;
        }

        return emitted + attenuation * scattered.color(background, world, depth - 1);
    }
}
