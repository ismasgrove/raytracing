use super::{Arc, HitRecord, Hittable, HittableList, Ray, Triangle, Vec3, XZRect, AABB};

pub struct Pyramid {
    faces: HittableList,
}

/*
    I don't want this to be permanently based on XZ-aligned rectangles.
    It was mostly the first and easiest implementation of a pyramid to come in mind
    I want to eventually look into whether I can generalize the XZ/YZ/XY modules into one structure
    For ease of use, mostly. The other option is to rely on rotation instances.
    But that's another matter I want to look at as well, as RotateX and RotateZ aren't yet implemented
    and since bounding_box isn't implemented for the Triangle module yet. it can't be used on any of its composites (at the moment, only pyramid)
    we'll see
*/

impl Pyramid {
    pub fn new(base: XZRect, apex: Vec3) -> Self {
        let mut faces = HittableList::new(vec![]);
        faces.add(Arc::new(Triangle::new(
            [
                apex,
                Vec3::new(base.x0, base.k, base.z0),
                Vec3::new(base.x0, base.k, base.z1),
            ],
            base.material.clone(),
        )));
        faces.add(Arc::new(Triangle::new(
            [
                apex,
                Vec3::new(base.x0, base.k, base.z0),
                Vec3::new(base.x1, base.k, base.z0),
            ],
            base.material.clone(),
        )));
        faces.add(Arc::new(Triangle::new(
            [
                apex,
                Vec3::new(base.x1, base.k, base.z0),
                Vec3::new(base.x1, base.k, base.z1),
            ],
            base.material.clone(),
        )));
        faces.add(Arc::new(Triangle::new(
            [
                apex,
                Vec3::new(base.x0, base.k, base.z1),
                Vec3::new(base.x1, base.k, base.z1),
            ],
            base.material.clone(),
        )));

        faces.add(Arc::new(base));

        Pyramid { faces }
    }
}

impl Hittable for Pyramid {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.faces.hit(r, t_min, t_max)
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.faces.bounding_box(t0, t1)
    }
}
