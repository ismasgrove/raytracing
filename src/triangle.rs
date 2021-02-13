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

        let upper_A_term = (vertices[1].y() - vertices[2].y()) * (p.x() - vertices[2].x())
            + (vertices[2].x() - vertices[1].x()) * (p.y() - vertices[2].y());
        let lower_A_term = (vertices[1].y() - vertices[2].y())
            * (vertices[0].x() - vertices[2].x())
            + (vertices[2].x() - vertices[1].x()) * (vertices[0].y() - vertices[2].y());

        let bary_A = upper_A_term / lower_A_term;

        let upper_B_term = (vertices[2].y() - vertices[0].y()) * (p.x() - vertices[2].x())
            + (vertices[0].x() - vertices[2].x()) * (p.y() - vertices[2].y());
        let lower_B_term = (vertices[1].y() - vertices[2].y())
            * (vertices[0].x() - vertices[2].x())
            + (vertices[2].x() - vertices[1].x()) * (vertices[0].y() - vertices[2].y());

        let bary_B = upper_B_term / lower_B_term;

        let bary_C = 1. - bary_A - bary_B;

        let (u, v) = (
            bary_A * vertices[0].x() + bary_B * vertices[1].x() + bary_C * vertices[2].x(),
            bary_A * vertices[0].y() + bary_B * vertices[1].y() + bary_C * vertices[2].y(),
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
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        None
    }
}
