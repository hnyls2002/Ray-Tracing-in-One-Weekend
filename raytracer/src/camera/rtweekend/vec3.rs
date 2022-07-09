use std::ops::{Add, Div, Mul, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, Neg};

use super::random_double;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2);
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs);
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self(self.0 / rhs, self.1 / rhs, self.2 / rhs);
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.0 * v.0 + u.1 * v.1 + u.2 * v.2
}

/*
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3(
        u.1 * v.2 - u.2 * v.1,
        u.2 * v.0 - u.0 * v.2,
        u.0 * v.1 - u.1 * v.0,
    )
}
*/

impl Vec3 {
    pub fn length(self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }
    pub fn unit_vec(self) -> Self {
        self / self.length()
    }
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }
    /*pub fn random_unit() -> Self {
        Vec3(
            random_double_unit(),
            random_double_unit(),
            random_double_unit(),
        )
    }*/
    pub fn random(min: f64, max: f64) -> Self {
        Vec3(
            random_double(min, max),
            random_double(min, max),
            random_double(min, max),
        )
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::random(-1.0, 1.0);
    loop {
        if p.length().powi(2) < 1.0 {
            break;
        }
        p = Vec3::random(-1.0, 1.0);
    }
    p
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit_vec()
}

pub type Point3 = Vec3;
pub type Color = Vec3;
