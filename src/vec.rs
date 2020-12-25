use super::utils;
use rand::Rng;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
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

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v = Self::random_from_range(-1., 1.);
            if v.squared_length() >= 1. {
                return v;
            } else {
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().normalize()
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
