use rand::{
    prelude::{SliceRandom, StdRng},
    Rng, SeedableRng,
};

use crate::basic::vec3::{dot, Color, Point3, Vec3};

use super::Texture;

const POINT_COUNT: usize = 256;

#[derive(Clone)]
pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    fn perlin_generater_perm() -> Vec<usize> {
        let mut rng = StdRng::seed_from_u64(1926);
        let mut ret: Vec<usize> = (0..POINT_COUNT).collect();
        ret.shuffle(&mut rng);
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

        let mut c: [Vec3; 8] = [Default::default(); 8];
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
                        self.ranvec[self.perm_x[((i + di) & 255) as usize]
                            ^ self.perm_y[((j + dj) & 255) as usize]
                            ^ self.perm_z[((k + dk) & 255) as usize]],
                    );
                }
            }
        }
        Perlin::trilinear_interp(c, u, v, w)
    }
    fn trilinear_interp(c: [Vec3; 8], u: f64, v: f64, w: f64) -> f64 {
        let uu = u.powi(2) * (3.0 - 2.0 * u);
        let vv = v.powi(2) * (3.0 - 2.0 * v);
        let ww = w.powi(2) * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        let c_val = |x, y, z| c[(x * 4 + y * 2 + z) as usize];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * dot(&c_val(i, j, k), &weight_v)
                }
            }
        }
        accum
    }
    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
}

impl Default for Perlin {
    fn default() -> Self {
        let mut arr = vec![];
        let mut rng = StdRng::seed_from_u64(817);
        for _i in 0..POINT_COUNT {
            arr.push(Vec3(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            ));
        }
        Self {
            ranvec: arr,
            perm_x: Perlin::perlin_generater_perm(),
            perm_y: Perlin::perlin_generater_perm(),
            perm_z: Perlin::perlin_generater_perm(),
        }
    }
}
#[derive(Default, Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new_by_sc(sc: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Default::default(),
            scale: sc,
        }
    }
}
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        // smoothed higer frequency
        //Vec3(1.0, 1.0, 1.0) * (1.0 + self.noise.noise(&(*p * self.scale))) * 0.5

        // turbulence
        //Vec3(1.0, 1.0, 1.0) * self.noise.turb(&(*p * self.scale), 7)

        // marbled texture
        let tmp = self.scale * p.2 + 10.0 * self.noise.turb(p, 7);
        Vec3(1.0, 1.0, 1.0) * (1.0 + tmp.sin()) * 0.5
    }
}
