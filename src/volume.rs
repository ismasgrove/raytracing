use super::{utils, Arc, HitRecord, Hittable, Material, Ray, Texture, Vec3, AABB};

pub struct ConstantMedium {
    neg_inv_density: f64,
    boundary: Arc<dyn Hittable>,
    phase_fn: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(density: f64, boundary: Arc<dyn Hittable>, tex: Arc<dyn Texture>) -> Self {
        ConstantMedium {
            neg_inv_density: -1. / density,
            boundary,
            phase_fn: Arc::new(Isotropic::new(tex)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut hit1) = self
            .boundary
            .hit(r, std::f64::NEG_INFINITY, std::f64::INFINITY)
        {
            if let Some(mut hit2) = self.boundary.hit(r, hit1.t + 0.0001, std::f64::INFINITY) {
                //some clamping
                if hit1.t < t_min {
                    hit1.t = t_min;
                }
                if hit2.t > t_max {
                    hit2.t = t_max;
                }

                if hit1.t >= hit2.t {
                    return None;
                }

                if hit1.t < 0. {
                    hit1.t = 0.;
                }

                let ray_length = r.direction().length();
                let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
                let hit_distance = self.neg_inv_density * utils::random_double().log(2.);

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = hit1.t + hit_distance / ray_length;
                Some(HitRecord::new(
                    t,
                    /*
                        these (u,v) coordinates will be ignored since I'm using a Solid texture
                        need to test how it behaves under other texture types
                    */
                    hit2.u,
                    hit2.v,
                    r.point(t),
                    Vec3::new(1., 0., 0.),
                    r,
                    &self.phase_fn,
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Isotropic { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        Some((
            self.albedo.value(rec.u, rec.v, &rec.p),
            Ray::new(rec.p, Vec3::random_in_unit_sphere(), Some(r_in.time())),
        ))
    }
}
