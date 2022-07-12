use std::ops::{Add, Div, Index, Mul, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, Neg};

use super::{random_double, random_double_unit};

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
    fn sub(self, rhs: Vec3) -> Self::Output {
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

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &f64 {
        if index == 0 {
            &self.0
        } else if index == 1 {
            &self.1
        } else {
            &self.2
        }
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.0 * v.0 + u.1 * v.1 + u.2 * v.2
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3(
        u.1 * v.2 - u.2 * v.1,
        u.2 * v.0 - u.0 * v.2,
        u.0 * v.1 - u.1 * v.0,
    )
}

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
    #[allow(dead_code)]
    pub fn random_unit() -> Self {
        Vec3(
            random_double_unit(),
            random_double_unit(),
            random_double_unit(),
        )
    }
    pub fn random(min: f64, max: f64) -> Self {
        Vec3(
            random_double(min, max),
            random_double(min, max),
            random_double(min, max),
        )
    }
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0.abs() < s && self.1.abs() < s && self.2.abs() < s
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

pub fn random_in_unit_disk() -> Vec3 {
    let ret: Vec3;
    loop {
        let p = Vec3(random_double(-1.0, 1.0), random_double(-1.0, 1.0), 0.0);
        if p.length() < 1.0 {
            ret = p;
            break;
        }
    }
    ret
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * dot(v, n) * 2.0
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&-*uv, n).min(1.0);
    let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
    let r_out_parallel = *n * -((1.0 - r_out_perp.length().powi(2)).abs().sqrt());
    r_out_perp + r_out_parallel
}

#[allow(dead_code)]
pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(&in_unit_sphere, normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;
