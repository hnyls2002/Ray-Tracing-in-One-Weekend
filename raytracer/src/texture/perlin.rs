use rand::{prelude::SliceRandom, thread_rng};

use crate::rtweekend::{random_double_unit, vec3::Point3};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    fn perlin_generater_perm() -> Vec<usize> {
        let mut ret: Vec<usize> = (0..POINT_COUNT).collect();
        ret.shuffle(&mut thread_rng());
        ret
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let i = ((4.0 * p.0).trunc() as i32 & 255) as usize;
        let j = ((4.0 * p.1).trunc() as i32 & 255) as usize;
        let k = ((4.0 * p.2).trunc() as i32 & 255) as usize;
        self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
}

impl Default for Perlin {
    fn default() -> Self {
        let mut arr = vec![];
        for _i in 0..POINT_COUNT {
            arr.push(random_double_unit());
        }
        Self {
            ranfloat: arr,
            perm_x: Perlin::perlin_generater_perm(),
            perm_y: Perlin::perlin_generater_perm(),
            perm_z: Perlin::perlin_generater_perm(),
        }
    }
}
