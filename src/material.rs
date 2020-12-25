use super::HitRecord;
use super::Ray;
use super::Vec3;

pub trait Material {
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

        (self.albedo, Ray::new(rec.p, scatter_direction), true)
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(col: Vec3) -> Self {
        Metal { albedo: col }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Vec3, Ray, bool) {
        let reflected = Vec3::reflect(&r_in.direction(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        let scatter = scattered.direction().dot(rec.normal) > 0.;
        (self.albedo, scattered, scatter)
    }
}
