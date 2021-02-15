use super::{HitRecord, Hittable, Ray, Vec3, AABB};

pub struct Plane {
    point: Vec3, // this point represents the distance from the origin
    normal: Vec3,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3) -> Self {
        Plane { point, normal }
    }
}

impl Hittable for Plane {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        None
    }
}
