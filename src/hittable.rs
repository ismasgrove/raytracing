use super::{material::Material, Direction, Position, Ray, AABB};
use std::ops::{Index, IndexMut};
use std::sync::Arc;

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}

pub struct HitRecord {
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub p: Position,
    pub normal: Direction,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        t: f64,
        u: f64,
        v: f64,
        p: Position,
        outward_normal: Direction,
        r: &Ray,
        material: &Arc<dyn Material>,
    ) -> Self {
        let front_face = r.direction().dot(outward_normal) < 0.;
        HitRecord {
            t,
            u,
            v,
            p,
            front_face,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            material: material.clone(),
        }
    }
}

pub struct HittableList {
    list: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Vec<Arc<dyn Hittable>>) -> Self {
        HittableList { list }
    }
    pub fn add(&mut self, obj: Arc<dyn Hittable>) {
        self.list.push(obj);
    }
    pub fn list(&self) -> &Vec<Arc<dyn Hittable>> {
        &self.list
    }
    pub fn len(&self) -> usize {
        self.list.len()
    }
}

impl Index<usize> for HittableList {
    type Output = Arc<dyn Hittable>;
    fn index(&self, i: usize) -> &Arc<dyn Hittable> {
        &self.list[i]
    }
}

impl IndexMut<usize> for HittableList {
    fn index_mut(&mut self, i: usize) -> &mut Arc<dyn Hittable> {
        &mut self.list[i]
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        /* */
        let mut rec = None;
        let mut closest_so_far = t_max;
        for item in self.list.iter() {
            let temp = item.hit(r, t_min, closest_so_far);
            if let Some(ref hit) = temp {
                closest_so_far = hit.t;
                rec = temp;
            }
        }

        rec
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.list.is_empty() {
            return None;
        }

        if let Some(temp) = self.list[0].bounding_box(t0, t1) {
            let mut accumulating_box = temp;
            for item in &self.list[1..] {
                if let Some(other_box) = item.bounding_box(t0, t1) {
                    accumulating_box = AABB::surrounding_box(other_box, accumulating_box);
                }
            }
            Some(accumulating_box)
        } else {
            None
        }
    }
}
