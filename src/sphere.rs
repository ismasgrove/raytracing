use super::hittable::{HitRecord, Hittable};
use super::Ray;
use super::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    type Output = Option<HitRecord>;
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center;
        let a = r.direction().squared_length();
        let b_half = oc.dot(r.direction());
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = b_half * b_half - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-b_half - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-b_half + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let outward_normal = (r.point(root) - self.center) / self.radius;

        Some(HitRecord::new(root, r.point(root), outward_normal, r))
    }
}
