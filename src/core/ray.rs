use super::{Color, Point, Vector};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn color(&self) -> Color {
        if crate::hit_sphere(Point::new(0.0, 0.0, -1.0), 0.5, self) {
            return Color::new(1.0, 0.0, 0.0);
        }

        let unit_direction = self.direction.unit_vector();

        let t = (unit_direction.y + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }
}
