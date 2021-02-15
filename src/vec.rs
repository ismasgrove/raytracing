use super::utils;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

/*
    Type aliases, to better differentiate between the variety of Vec3 types throughout the code
    TODO: make Vec3 private, replace with aliases accordingly
*/

pub type Color = Vec3;
pub type Position = Vec3;
pub type Direction = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn new_diagonal(x: f64) -> Self {
        Self { e: [x, x, x] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn r(&self) -> f64 {
        self.e[0]
    }
    pub fn g(&self) -> f64 {
        self.e[1]
    }
    pub fn b(&self) -> f64 {
        self.e[2]
    }

    pub fn index(&self, i: i32) -> f64 {
        self.e[i as usize]
    }

    pub fn length(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }
    pub fn squared_length(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn dot(&self, _rhs: Self) -> f64 {
        self.e[0] * _rhs.e[0] + self.e[1] * _rhs.e[1] + self.e[2] * _rhs.e[2]
    }

    pub fn cross(&self, _rhs: Self) -> Self {
        Self {
            e: [
                self.e[1] * _rhs.e[2] - self.e[2] * _rhs.e[1],
                self.e[2] * _rhs.e[0] - self.e[0] * _rhs.e[2],
                self.e[0] * _rhs.e[1] - self.e[1] * _rhs.e[0],
            ],
        }
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    pub fn random_from_range(min: f64, max: f64) -> Self {
        Self::new(
            utils::random_from_range(min, max),
            utils::random_from_range(min, max),
            utils::random_from_range(min, max),
        )
    }
    pub fn random() -> Self {
        Self::new(
            utils::random_double(),
            utils::random_double(),
            utils::random_double(),
        )
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Self::new(
                utils::random_from_range(-1., 1.),
                utils::random_from_range(-1., 1.),
                0.,
            );
            if p.squared_length() >= 1. {
                continue;
            } else {
                return p;
            };
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v = Self::random_from_range(-1., 1.);
            if v.squared_length() <= 1. {
                return v;
            } else {
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().normalize()

        /*
            equal-area projection
            https://math.stackexchange.com/questions/44689/how-to-find-a-random-axis-or-unit-vector-in-3d

        let (a, z) = (
            utils::random_from_range(0., 2. * std::f64::consts::PI),
            utils::random_from_range(-1., 1.),
        );

        let r = (1. - z * z).sqrt();

        Vec3::new(r * (a.cos()), r * (a.sin()), z)
        */
    }

    pub fn random_in_hemisphere(normal: Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0. {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        (self.e[0].abs() < f64::EPSILON)
            && (self.e[1].abs() < f64::EPSILON)
            && (self.e[2].abs() < f64::EPSILON)
    }

    pub fn reflect(v: &Self, n: &Self) -> Self {
        *v - 2. * v.dot(*n) * *n
    }
    pub fn refract(uv: &Self, n: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = n.dot(-*uv).min(1.);
        let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_par = -(1. - r_out_perp.squared_length()).abs().sqrt() * *n;
        r_out_perp + r_out_par
    }

    pub fn sqrt(&self) -> Self {
        Vec3::new(self.x().sqrt(), self.y().sqrt(), self.z().sqrt())
    }

    pub fn abs(&self) -> Self {
        Vec3::new(self.x().abs(), self.x().abs(), self.x().abs())
    }

    pub fn into_bytes(&self, n_samples: i32) -> [u8; 3] {
        let scale = 1. / n_samples as f64;
        let r = (255.99 * utils::clamp((self.r() * scale).sqrt(), 0., 0.999)) as u8;
        let g = (255.99 * utils::clamp((self.g() * scale).sqrt(), 0., 0.999)) as u8;
        let b = (255.99 * utils::clamp((self.b() * scale).sqrt(), 0., 0.999)) as u8;
        [r, g, b]
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + _rhs.e[0],
                self.e[1] + _rhs.e[1],
                self.e[2] + _rhs.e[2],
            ],
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - _rhs.e[0],
                self.e[1] - _rhs.e[1],
                self.e[2] - _rhs.e[2],
            ],
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * _rhs.e[0],
                self.e[1] * _rhs.e[1],
                self.e[2] * _rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: f64) -> Self {
        Vec3 {
            e: [self.e[0] * _rhs, self.e[1] * _rhs, self.e[2] * _rhs],
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, _lhs: Vec3) -> Vec3 {
        Vec3 {
            e: [_lhs[0] * self, _lhs[1] * self, _lhs[2] * self],
        }
    }
}

impl ops::Div for Vec3 {
    type Output = Vec3;
    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] / _rhs.e[0],
                self.e[1] / _rhs.e[1],
                self.e[2] / _rhs.e[2],
            ],
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, _rhs: f64) -> Self {
        Vec3 {
            e: [self.e[0] / _rhs, self.e[1] / _rhs, self.e[2] / _rhs],
        }
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;
    fn div(self, _lhs: Vec3) -> Vec3 {
        Vec3 {
            e: [_lhs[0] / self, _lhs[1] / self, _lhs[2] / self],
        }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, _rhs: Self) {
        self.e[0] += _rhs.e[0];
        self.e[1] += _rhs.e[1];
        self.e[2] += _rhs.e[2];
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, _rhs: Self) {
        self.e[0] -= _rhs.e[0];
        self.e[1] -= _rhs.e[1];
        self.e[2] -= _rhs.e[2];
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, _rhs: Self) {
        self.e[0] *= _rhs.e[0];
        self.e[1] *= _rhs.e[1];
        self.e[2] *= _rhs.e[2];
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, _rhs: f64) {
        self.e[0] *= _rhs;
        self.e[1] *= _rhs;
        self.e[2] *= _rhs;
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, _rhs: Self) {
        self.e[0] /= _rhs.e[0];
        self.e[1] /= _rhs.e[1];
        self.e[2] /= _rhs.e[2];
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, _rhs: f64) {
        self.e[0] /= _rhs;
        self.e[1] /= _rhs;
        self.e[2] /= _rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}
