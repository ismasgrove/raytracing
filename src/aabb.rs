use super::{Ray, Vec3};

#[derive(Copy, Clone)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        AABB { min: a, max: b }
    }
    pub fn min(&self) -> Vec3 {
        self.min
    }
    pub fn max(&self) -> Vec3 {
        self.max
    }
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for i in 0..3 {
            let inverse_dir = 1. / r.direction()[i];

            let (mut t0, mut t1) = if inverse_dir < 0. {
                (
                    (self.min[i] - r.origin()[i]) * inverse_dir,
                    (self.max[i] - r.origin()[i]) * inverse_dir,
                )
            } else {
                (
                    (self.max[i] - r.origin()[i]) * inverse_dir,
                    (self.min[i] - r.origin()[i]) * inverse_dir,
                )
            };
            t0 = if t0 > t_min { t0 } else { t_min };
            t1 = if t1 < t_max { t1 } else { t_max };
            if t1 <= t0 {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> Self {
        let small = Vec3::new(
            box0.min().x().min(box1.min().x()),
            box0.min().y().min(box1.min().y()),
            box0.min().z().min(box1.min().z()),
        );
        let big = Vec3::new(
            box0.max().x().max(box1.max().x()),
            box0.max().y().max(box1.max().y()),
            box0.max().z().max(box1.max().z()),
        );

        AABB::new(small, big)
    }
}
