use super::{Arc, HitRecord, Hittable, HittableList, Material, Ray, Vec3, AABB};

pub struct XYRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: Arc<dyn Material>,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        XYRect {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            None
        } else {
            let x = r.origin().x() + t * r.direction().x();
            let y = r.origin().y() + t * r.direction().y();
            if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
                None
            } else {
                let (u, v) = (
                    (x - self.x0) / (self.x1 - self.x0),
                    (y - self.y0) / (self.y1 - self.y0),
                );
                Some(HitRecord::new(
                    t,
                    u,
                    v,
                    r.point(t),
                    Vec3::new(0., 0., 1.),
                    r,
                    &self.material,
                ))
            }
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

pub struct YZRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Arc<dyn Material>,
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        YZRect {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Hittable for YZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max {
            None
        } else {
            let y = r.origin().y() + t * r.direction().y();
            let z = r.origin().z() + t * r.direction().z();
            if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
                None
            } else {
                let (u, v) = (
                    (y - self.y0) / (self.y1 - self.y0),
                    (z - self.z0) / (self.z1 - self.z0),
                );
                Some(HitRecord::new(
                    t,
                    u,
                    v,
                    r.point(t),
                    Vec3::new(1., 0., 0.),
                    r,
                    &self.material,
                ))
            }
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}

pub struct XZRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Arc<dyn Material>,
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        XZRect {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Hittable for XZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max {
            None
        } else {
            let x = r.origin().x() + t * r.direction().x();
            let z = r.origin().z() + t * r.direction().z();
            if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
                None
            } else {
                let (u, v) = (
                    (x - self.x0) / (self.x1 - self.x0),
                    (z - self.z0) / (self.z1 - self.z0),
                );
                Some(HitRecord::new(
                    t,
                    u,
                    v,
                    r.point(t),
                    Vec3::new(0., 1., 0.),
                    r,
                    &self.material,
                ))
            }
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k - 0.0001, self.z1),
        ))
    }
}

pub struct Cuboid {
    p0: Vec3,
    p1: Vec3,
    sides: HittableList,
}

impl Cuboid {
    pub fn new(p0: Vec3, p1: Vec3, material: Arc<dyn Material>) -> Self {
        let sides = HittableList::new(vec![
            Arc::new(XYRect::new(
                p0.x(),
                p1.x(),
                p0.y(),
                p1.y(),
                p1.z(),
                material.clone(),
            )),
            Arc::new(XYRect::new(
                p0.x(),
                p1.x(),
                p0.y(),
                p1.y(),
                p0.z(),
                material.clone(),
            )),
            Arc::new(XZRect::new(
                p0.x(),
                p1.x(),
                p0.z(),
                p1.z(),
                p1.y(),
                material.clone(),
            )),
            Arc::new(XZRect::new(
                p0.x(),
                p1.x(),
                p0.z(),
                p1.z(),
                p0.y(),
                material.clone(),
            )),
            Arc::new(YZRect::new(
                p0.y(),
                p1.y(),
                p0.z(),
                p1.z(),
                p1.x(),
                material.clone(),
            )),
            Arc::new(YZRect::new(
                p0.y(),
                p1.y(),
                p0.z(),
                p1.z(),
                p0.x(),
                material,
            )),
        ]);

        Cuboid { p0, p1, sides }
    }
}

impl Hittable for Cuboid {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.sides.bounding_box(t0, t1)
    }
}
