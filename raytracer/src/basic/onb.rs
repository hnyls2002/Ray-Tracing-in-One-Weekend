use std::ops::Index;

use super::vec3::{cross, Vec3};

#[derive(Default)]
pub struct Onb {
    pub axis: [Vec3; 3],
}

impl Index<usize> for Onb {
    type Output = Vec3;
    fn index(&self, index: usize) -> &Self::Output {
        &self.axis[index]
    }
}

impl Onb {
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }
    #[allow(dead_code)]
    pub fn local_by_abc(&self, a: f64, b: f64, c: f64) -> Vec3 {
        self.u() * a + self.v() * b + self.w() * c
    }
    pub fn local_by_vec3(&self, v: Vec3) -> Vec3 {
        self.u() * v.0 + self.v() * v.1 + self.w() * v.2
    }
    pub fn build_from_w(n: &Vec3) -> Self {
        let mut ret = Self {
            axis: Default::default(),
        };
        ret.axis[2] = n.unit_vec();
        let a = if ret.w().0.abs() > 0.9 {
            Vec3(0.0, 1.0, 0.0)
        } else {
            Vec3(1.0, 0.0, 0.0)
        };
        ret.axis[1] = cross(&ret.w(), &a).unit_vec();
        ret.axis[0] = cross(&ret.w(), &ret.v());
        ret
    }
}
