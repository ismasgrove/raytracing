use super::{Position, Ray};

#[derive(Copy, Clone)]
pub struct AABB {
    min: Position,
    max: Position,
}

impl AABB {
    pub fn new(min: Position, max: Position) -> Self {
        AABB { min, max }
    }
    pub fn min(&self) -> Position {
        self.min
    }
    pub fn max(&self) -> Position {
        self.max
    }
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let (mut t_min, mut t_max) = (t_min, t_max);
        for i in 0..3 {
            let inverse_dir = 1. / r.direction()[i];

            let (t0, t1) = if inverse_dir >= 0. {
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

            t_min = t0.max(t_min);
            t_max = t1.min(t_max);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> Self {
        let small = Position::new(
            box0.min().x().min(box1.min().x()),
            box0.min().y().min(box1.min().y()),
            box0.min().z().min(box1.min().z()),
        );

        let big = Position::new(
            box0.max().x().max(box1.max().x()),
            box0.max().y().max(box1.max().y()),
            box0.max().z().max(box1.max().z()),
        );

        AABB::new(small, big)
    }
}
