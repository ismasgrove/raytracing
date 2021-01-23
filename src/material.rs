use super::utils;
use super::HitRecord;
use super::Ray;
use super::Vec3;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Vec3, Ray, bool);
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(col: Vec3) -> Self {
        Lambertian { albedo: col }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Vec3, Ray, bool) {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        (
            self.albedo,
            Ray::new(rec.p, scatter_direction, Some(r_in.time())),
            true,
        )
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzziness: f64) -> Self {
        Metal { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Vec3, Ray, bool) {
        let reflected = Vec3::reflect(&r_in.direction(), &rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzziness * Vec3::random_in_unit_sphere(),
            Some(r_in.time()),
        );
        let scatter = scattered.direction().dot(rec.normal) > 0.;
        (self.albedo, scattered, scatter)
    }
}

pub struct Dielectric {
    refr_index: f64,
}

impl Dielectric {
    pub fn new(refr_index: f64) -> Self {
        Dielectric { refr_index }
    }
    fn reflectance(cosine: f64, refr_index: f64) -> f64 {
        let mut r0 = (1. - refr_index) / (1. + refr_index);
        r0 *= r0;
        r0 + (1. - r0) * (1. - cosine).powf(5.)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Vec3, Ray, bool) {
        let refr_ratio = if rec.front_face {
            1. / self.refr_index
        } else {
            self.refr_index
        };
        let unit_dir = r_in.direction().normalize();
        let cos_theta = 1.0_f64.min(rec.normal.dot(-unit_dir));
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let dir = if (refr_ratio * sin_theta > 1.)
            || Dielectric::reflectance(cos_theta, refr_ratio) > utils::random_double()
        {
            Vec3::reflect(&unit_dir, &rec.normal)
        } else {
            Vec3::refract(&unit_dir, &rec.normal, refr_ratio)
        };
        (
            Vec3::new(1., 1., 1.),
            Ray::new(rec.p, dir, Some(r_in.time())),
            true,
        )
    }
}
