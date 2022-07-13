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
    #[allow(clippy::many_single_char_names)]
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.0 - p.0.floor();
        let v = p.1 - p.1.floor();
        let w = p.2 - p.2.floor();

        let i = p.0.floor() as i32;
        let j = p.1.floor() as i32;
        let k = p.2.floor() as i32;

        let mut c: [f64; 8] = [0.0; 8];
        let mut c_ass = |x, y, z, v| {
            c[(x * 4 + y * 2 + z) as usize] = v;
        };

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c_ass(
                        di,
                        dj,
                        dk,
                        self.ranfloat[self.perm_x[((i + di) & 255) as usize]
                            ^ self.perm_y[((j + dj) & 255) as usize]
                            ^ self.perm_z[((k + dk) & 255) as usize]],
                    );
                }
            }
        }
        Perlin::trilinear_interp(c, u, v, w)
    }
    fn trilinear_interp(c: [f64; 8], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        let c_val = |x, y, z| c[(x * 4 + y * 2 + z) as usize];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * c_val(i, j, k)
                }
            }
        }
        accum
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
