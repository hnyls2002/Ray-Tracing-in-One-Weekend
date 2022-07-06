mod vec3;

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Ray {
            orig: origin,
            dir: direction,
        }
    }
    pub fn at(t: f64) -> Point3 {
        orig + dir * t
    }
}
