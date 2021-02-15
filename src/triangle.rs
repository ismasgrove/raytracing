use super::{Arc, HitRecord, Hittable, Material, Ray, Vec3, AABB};
use std::f64;

pub struct Triangle {
    vertices: [Vec3; 3],
    material: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(vertices: [Vec3; 3], material: Arc<dyn Material>) -> Self {
        Triangle { vertices, material }
    }

    fn barycentric(p: Vec3, vertices: [Vec3; 3]) -> (f64, f64) {
        /*
            barycentric lerp
        */

        let upper_a_term = (vertices[1].y() - vertices[2].y()) * (p.x() - vertices[2].x())
            + (vertices[2].x() - vertices[1].x()) * (p.y() - vertices[2].y());
        let lower_a_term = (vertices[1].y() - vertices[2].y())
            * (vertices[0].x() - vertices[2].x())
            + (vertices[2].x() - vertices[1].x()) * (vertices[0].y() - vertices[2].y());

        let bary_a = upper_a_term / lower_a_term;

        let upper_b_term = (vertices[2].y() - vertices[0].y()) * (p.x() - vertices[2].x())
            + (vertices[0].x() - vertices[2].x()) * (p.y() - vertices[2].y());
        let lower_b_term = (vertices[1].y() - vertices[2].y())
            * (vertices[0].x() - vertices[2].x())
            + (vertices[2].x() - vertices[1].x()) * (vertices[0].y() - vertices[2].y());

        let bary_b = upper_b_term / lower_b_term;

        let bary_c = 1. - bary_a - bary_b;

        let (u, v) = (
            bary_a * vertices[0].x() + bary_b * vertices[1].x() + bary_c * vertices[2].x(),
            bary_a * vertices[0].y() + bary_b * vertices[1].y() + bary_c * vertices[2].y(),
        );

        (u, v)
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        /*
            muoller-trumbore intersection algorithm
        */

        let (edge1, edge2) = (
            self.vertices[1] - self.vertices[0],
            self.vertices[2] - self.vertices[0],
        );
        let h = r.direction().cross(edge2);
        let a = edge1.dot(h);

        if a > -f64::EPSILON && a < f64::EPSILON {
            return None;
        }

        let f = 1. / a;
        let s = r.origin() - self.vertices[0];
        let u = f * s.dot(h);
        if u < 0. || u > 1. {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * r.direction().dot(q);
        if v < 0. || u + v > 1. {
            return None;
        }

        let t = f * edge2.dot(q);

        let outward_normal = edge1.cross(edge2).normalize();

        if t > f64::EPSILON && t < t_max && t > t_min {
            Some(HitRecord::new(
                t,
                u,
                v,
                r.point(t),
                outward_normal,
                &r,
                &self.material.clone(),
            ))
        } else {
            None
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        let max_x = self.vertices[0]
            .x()
            .max(self.vertices[1].x().max(self.vertices[2].x()));
        let max_y = self.vertices[0]
            .y()
            .max(self.vertices[1].y().max(self.vertices[2].y()));
        let max_z = self.vertices[0]
            .z()
            .max(self.vertices[1].z().max(self.vertices[2].z()));
        let min_x = self.vertices[0]
            .x()
            .min(self.vertices[1].x().min(self.vertices[2].x()));
        let min_y = self.vertices[0]
            .y()
            .min(self.vertices[1].y().min(self.vertices[2].y()));
        let min_z = self.vertices[0]
            .z()
            .min(self.vertices[1].z().min(self.vertices[2].z()));

        Some(AABB::new(
            Vec3::new(min_x, min_y, min_z),
            Vec3::new(max_x, max_y, max_z),
        ))
    }
}
