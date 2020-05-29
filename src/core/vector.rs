use std::fmt;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point = Vector;
pub type Color = Vector;

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, v: Vector) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    fn cross(&self, v: Vector) -> Self {
        Self {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.z,
        }
    }

    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }

    pub fn write_color(&self) -> String {
        let new = self * 255.999;
        format!("{} {} {}", new.x as u64, new.y as u64, new.z as u64)
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
