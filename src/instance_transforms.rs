use std::f64;

use super::{utils, Arc, HitRecord, Hittable, Ray, Vec3, AABB};

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Translate { object, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(r.origin() - self.offset, r.direction(), Some(r.time()));
        if let Some(hit) = self.object.hit(&moved_ray, t_min, t_max) {
            return Some(HitRecord::new(
                hit.t,
                hit.u,
                hit.v,
                hit.p + self.offset,
                hit.normal,
                &moved_ray,
                &hit.material,
            ));
        }
        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if let Some(pre_offset) = self.object.bounding_box(t0, t1) {
            Some(AABB::new(
                pre_offset.min() + self.offset,
                pre_offset.max() + self.offset,
            ))
        } else {
            None
        }
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = utils::degrees_to_radians(angle);
        let (sin_theta, cos_theta) = (radians.sin(), radians.cos());
        let bbox = object.bounding_box(0., 1.);
        let (mut min, mut max) = (
            Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        );

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.unwrap().max().x()
                        + (1 - i) as f64 * bbox.unwrap().min().x();
                    let y = i as f64 * bbox.unwrap().max().y()
                        + (1 - j) as f64 * bbox.unwrap().min().y();
                    let z = i as f64 * bbox.unwrap().max().y()
                        + (i - k) as f64 * bbox.unwrap().min().z();

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let test = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = min[c].min(test[c]);
                        max[c] = max[c].max(test[c]);
                    }
                }
            }
        }

        RotateY {
            object,
            sin_theta,
            cos_theta,
            bbox: Some(AABB::new(min, max)),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (mut origin, mut direction) = (r.origin(), r.direction());
        /*
        todo: cleanup this part
        */
        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];
        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_ray = Ray::new(origin, direction, Some(r.time()));
        if let Some(hit) = self.object.hit(&rotated_ray, t_min, t_max) {
            let (mut p, mut normal) = (hit.p, hit.normal);
            p[0] = self.cos_theta * hit.p[0] + self.sin_theta * hit.p[2];
            p[2] = -self.sin_theta * hit.p[0] + self.cos_theta * hit.p[2];
            normal[0] = self.cos_theta * hit.normal[0] + self.sin_theta * hit.normal[2];
            normal[2] = -self.sin_theta * hit.normal[0] + self.cos_theta * hit.normal[2];
            Some(HitRecord::new(
                hit.t,
                hit.u,
                hit.v,
                p,
                normal,
                &rotated_ray,
                &hit.material,
            ))
        } else {
            None
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        self.bbox
    }
}
