use super::{Color, Point, Vector};
use crate::objects::Object;

use std::f64::INFINITY;

use rand::prelude::*;

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

    pub fn color<T: Object>(
        &self,
        background: Color,
        world: &T,
        depth: i8,
        rng: &mut ThreadRng,
    ) -> Color {
        if depth <= 0 {
            return Color::from(0);
        }

        if let Some(rec) = world.hit(self, 0.001..INFINITY, rng) {
            let emitted = rec.material.emitted(rec.u, rec.v, rec.p);

            match rec.material.scatter(self, &rec, rng) {
                Some((attenuation, scattered)) => {
                    emitted + attenuation * scattered.color(background, world, depth - 1, rng)
                }
                None => emitted,
            }
        } else {
            background
        }
    }
}
