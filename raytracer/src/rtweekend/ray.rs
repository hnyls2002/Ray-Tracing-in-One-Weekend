use super::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub tm: f64,
}

impl Ray {
    pub fn at(self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }
}
