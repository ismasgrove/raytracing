use super::hittable::{HitRecord, Hittable};
use super::Material;
use super::Ray;
use super::Vec3;

use std::sync::Arc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    radius: f64,
    material: Arc<dyn Material>,
    t0: f64,
    t1: f64,
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        radius: f64,
        material: Arc<dyn Material>,
        t0: f64,
        t1: f64,
    ) -> Self {
        Self {
            center0,
            center1,
            radius,
            material,
            t0,
            t1,
        }
    }
    pub fn center(&self, time: f64) -> Vec3 {
        self.center0 + ((time - self.t0) / (self.t1 - self.t0) * (self.center1 - self.center0))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center(r.time());
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

        let outward_normal = (r.point(root) - self.center(r.time())) / self.radius;

        Some(HitRecord::new(
            root,
            r.point(root),
            outward_normal,
            r,
            &self.material,
        ))
    }
}

impl Hittable for Sphere {
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

        Some(HitRecord::new(
            root,
            r.point(root),
            outward_normal,
            r,
            &self.material,
        ))
    }
}
