use super::Ray;
use super::Vec3;
use std::rc::Rc;

pub trait Hittable {
    type Output: ?Sized;
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, outward_normal: Vec3, r: &Ray) -> Self {
        let front_face = r.direction().dot(outward_normal) < 0.;
        HitRecord {
            t,
            p,
            front_face,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
        }
    }
}

pub struct HittableList {
    list: Vec<Rc<dyn Hittable<Output = Option<HitRecord>>>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { list: vec![] }
    }
    pub fn add(&mut self, obj: Rc<dyn Hittable<Output = Option<HitRecord>>>) {
        self.list.push(obj);
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }
}

impl Hittable for HittableList {
    type Output = Option<HitRecord>;
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        /*for item in self.list.iter() {
            let temp = item.hit(r, t_min, closest_so_far);
            match temp {
                Some(hit) => {
                    closest_so_far = hit.t;
                    rec = temp;
                }
                None => (),
            }
        }*/

        for item in self.list.iter() {
            let temp = item.hit(r, t_min, closest_so_far);
            if let Some(hit) = temp {
                closest_so_far = hit.t;
                rec = temp;
            }
        }

        rec
    }
}
