use super::Vec3;

pub struct Ray {
    a: Vec3,
    b: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3, time: Option<f64>) -> Self {
        Ray {
            a,
            b,
            time: time.unwrap_or(0.),
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn point(&self, t: f64) -> Vec3 {
        self.a + t * self.b
    }

    pub fn time(&self) -> f64 {
        self.time
    }
}
