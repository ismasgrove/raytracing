use super::{Arc, HitRecord, Hittable, Material, Ray, Vec3, AABB};

pub struct Plane {
    point: Vec3, // this point represents the distance from the origin
    normal: Vec3,
    material: Arc<dyn Material>,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Arc<dyn Material>) -> Self {
        Plane {
            point,
            normal,
            material,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let denominator = self.normal.dot(r.direction()); // for the formula t = [self.point - ray_origin] * plane_normal / ray_dir * plane_normal
        if denominator < -f64::EPSILON {
            let v = self.point - r.origin();
            let t = v.dot(self.normal) / denominator;
            if t > t_min && t < t_max {
                return Some(HitRecord::new(
                    t,
                    0., // filler
                    0., // filler
                    r.point(t),
                    self.normal,
                    r,
                    &self.material,
                ));
            }
        }
        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        None
    }
}
