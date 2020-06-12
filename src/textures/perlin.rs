use crate::core::{Point, Vector};

use rand::seq::SliceRandom;
use rand::thread_rng;

const POINT_COUNT: usize = 256;

#[derive(Clone)]
pub struct Perlin {
    ranvec: Vec<Vector>,
    perm_x: Vec<u64>,
    perm_y: Vec<u64>,
    perm_z: Vec<u64>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranvec = vec![];

        for _ in 0..POINT_COUNT {
            ranvec.push(Vector::random_in(-1.0, 1.0).unit_vector());
        }

        let perm_x = Self::generate_perm();
        let perm_y = Self::generate_perm();
        let perm_z = Self::generate_perm();

        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Point) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i64;
        let j = p.y().floor() as i64;
        let k = p.z().floor() as i64;

        let mut c: [[[Vector; 2]; 2]; 2] = [[[Vector::from(0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[((i + di as i64) & 255) as usize]
                        ^ self.perm_y[((j + dj as i64) & 255) as usize]
                        ^ self.perm_z[((k + dk as i64) & 255) as usize])
                        as usize];
                }
            }
        }

        Self::trilinear_intep(c, u, v, w)
    }

    pub fn turb(&self, p: Point, depth: i64) -> f64 {
        let mut acc = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        acc.abs()
    }

    fn generate_perm() -> Vec<u64> {
        let mut p = vec![];

        for i in 0..POINT_COUNT {
            p.push(i as u64);
        }

        let mut rng = thread_rng();
        p.shuffle(&mut rng);
        p
    }

    fn trilinear_intep(c: [[[Vector; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut acc = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;
                    let weight = Vector::new(u - i, v - j, w - k);

                    acc += (i * uu + (1.0 - i) * (1.0 - uu))
                        * (j * vv + (1.0 - j) * (1.0 - vv))
                        * (k * ww + (1.0 - k) * (1.0 - ww))
                        * c[i as usize][j as usize][k as usize].dot(weight);
                }
            }
        }

        acc
    }
}
