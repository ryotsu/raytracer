use crate::utils::clamp;

use std::f64::consts::PI;
use std::fmt;
use std::ops;

use rand::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point = Vector;
pub type Color = Vector;

impl Vector {
    pub fn new<T: Into<f64>, U: Into<f64>, V: Into<f64>>(x: T, y: U, z: V) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn from<T: Into<f64> + Copy>(a: T) -> Self {
        Self::new(a, a, a)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, v: Vector) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, v: Vector) -> Self {
        Self {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }

    pub fn reflect(&self, normal: Self) -> Self {
        self - normal * self.dot(normal) * 2.0
    }

    pub fn refract(&self, normal: Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-*self).dot(normal);
        let r_out_parallel = (*self + normal * cos_theta) * etai_over_etat;
        let r_out_perpendicular = normal * -(1.0 - r_out_parallel.length_squared()).sqrt();
        r_out_parallel + r_out_perpendicular
    }

    pub fn random_in(low: f64, high: f64, rng: &mut ThreadRng) -> Self {
        Self {
            x: rng.gen_range(low, high),
            y: rng.gen_range(low, high),
            z: rng.gen_range(low, high),
        }
    }

    pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Self {
        loop {
            let p = rng.gen::<Vector>() * 2.0 - Self::from(1);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector(rng: &mut ThreadRng) -> Self {
        let a = rng.gen_range(0.0, 2.0 * PI);
        let z: f64 = rng.gen_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Self::new(r * a.cos(), r * a.sin(), z)
    }

    pub fn random_in_hemisphere(normal: Self, rng: &mut ThreadRng) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere(rng);
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Self {
        loop {
            let p = Self::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) * 2.0 - Self::new(1, 1, 0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn write_color(&self, samples_per_pixel: usize) -> String {
        let scale = 1.0 / samples_per_pixel as f64;
        let r = (self.x * scale).sqrt();
        let g = (self.y * scale).sqrt();
        let b = (self.z * scale).sqrt();

        format!(
            "{} {} {}",
            (256.0 * clamp(r, 0.0, 0.999)) as u8,
            (256.0 * clamp(g, 0.0, 0.999)) as u8,
            (256.0 * clamp(b, 0.0, 0.999)) as u8
        )
    }
}

impl ops::Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }
}

impl ops::IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => &mut self.z,
        }
    }
}

impl ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Sub<Vector> for &Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::MulAssign for Vector {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl ops::Mul for Vector {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<f64> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl ops::Div<f64> for Vector {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::Div<f64> for &Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Distribution<Vector> for rand::distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector {
        Vector {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }
}
