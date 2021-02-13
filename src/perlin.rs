use super::{utils, Vec3};

pub struct Perlin {
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
    ranvec: Vec<Vec3>,
}

impl Perlin {
    pub fn new() -> Self {
        let point_count: usize = 256;
        Perlin {
            perm_x: Perlin::perlin_generate_perm(point_count),
            perm_y: Perlin::perlin_generate_perm(point_count),
            perm_z: Perlin::perlin_generate_perm(point_count),
            ranvec: (0..point_count)
                .map(|_| Vec3::random_unit_vector())
                .collect(),
        }
    }
    fn perlin_generate_perm(point_count: usize) -> Vec<usize> {
        let mut p = vec![];
        for i in 0..point_count {
            p.push(i);
        }

        Perlin::permute(&mut p, point_count);
        p
    }

    fn permute(p: &mut Vec<usize>, n: usize) {
        for i in (1..n).rev() {
            let target = utils::random_int(0, i as i32) as usize;
            p.swap(i, target);
            //p.shuffle(&mut thread_rng());
        }
    }

    fn trilinear_interpolation(c: Vec<Vec<Vec<f64>>>, u: f64, v: f64, w: f64) -> f64 {
        // hermite cube, but here we don't need the original values
        let u = u * u * (3. - 2. * u);
        let v = v * v * (3. - 2. * v);
        let w = w * w * (3. - 2. * w);
        let mut accumulator = 0.;
        for i in 0..=1 {
            for j in 0..=1 {
                for k in 0..=1 {
                    accumulator += (i as f64 * u + ((1 - i) as f64) * (1. - u))
                        * (j as f64 * v + ((1 - j) as f64) * (1. - v))
                        * (k as f64 * w + ((1 - k) as f64) * (1. - w))
                        * c[i][j][k];
                }
            }
        }

        accumulator
    }

    fn perlin_interpolation(c: Vec<Vec<Vec<Vec3>>>, u: f64, v: f64, w: f64) -> f64 {
        let hermite_u = u * u * (3. - 2. * u);
        let hermite_v = v * v * (3. - 2. * v);
        let hermite_w = w * w * (3. - 2. * w);
        let mut accumulator = 0.;
        for i in 0..=1 {
            for j in 0..=1 {
                for k in 0..=1 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accumulator += (i as f64 * hermite_u + ((1 - i) as f64) * (1. - hermite_u))
                        * (j as f64 * hermite_v + ((1 - j) as f64) * (1. - hermite_v))
                        * (k as f64 * hermite_w + ((1 - k) as f64) * (1. - hermite_w))
                        * c[i][j][k].dot(weight_v);
                }
            }
        }

        accumulator
    }

    pub fn generate_noise(&self, p: &Vec3) -> f64 {
        let (u, v, w) = (
            p.x() - p.x().floor(),
            p.y() - p.y().floor(),
            p.z() - p.z().floor(),
        );
        let (i, j, k) = (
            ((p.x().floor() as i32) & 255) as usize,
            ((p.y().floor() as i32) & 255) as usize,
            ((p.z().floor() as i32) & 255) as usize,
        );
        let mut c = vec![vec![vec![Vec3::new_diagonal(0.); 2]; 2]; 2];

        for ti in 0..=1 {
            for tj in 0..=1 {
                for tk in 0..=1 {
                    c[ti][tj][tk] = self.ranvec[self.perm_x[(i + ti) & 255]
                        ^ self.perm_y[(j + tj) & 255]
                        ^ self.perm_z[(k + tk) & 255]];
                }
            }
        }

        Perlin::perlin_interpolation(c, u, v, w)
    }

    pub fn turbulence(&self, p: &Vec3) -> f64 {
        const DEPTH: i32 = 7;
        let (mut accumulator, mut weight) = (0., 1.);
        let mut p_mut_copy = *p;
        for _ in 0..DEPTH {
            accumulator += weight * self.generate_noise(&p_mut_copy);
            weight *= 0.5;
            p_mut_copy *= 2.;
        }

        accumulator.abs()
    }
}
