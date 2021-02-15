use super::{texture, utils, Arc, Color, HitRecord, Position, Ray, Texture, Vec3};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Color {
        Vec3::new(0., 0., 0.)
    }
}

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(col: Color) -> Self {
        Lambertian {
            albedo: Arc::new(texture::Solid::color_vec3(col)),
        }
    }
    pub fn textured(t: Arc<dyn Texture>) -> Self {
        Lambertian { albedo: t }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((
            self.albedo.value(rec.u, rec.v, &rec.p),
            Ray::new(rec.p, 0.5 * scatter_direction, Some(r_in.time())),
        ))
    }
}

pub struct Metal {
    albedo: Color,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Self {
        Metal { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&r_in.direction().normalize(), &rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzziness * Vec3::random_in_unit_sphere(),
            Some(r_in.time()),
        );
        if scattered.direction().dot(rec.normal) > 0. {
            Some((self.albedo, scattered))
        } else {
            None
        }
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
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

        Some((
            Vec3::new(1., 1., 1.),
            Ray::new(rec.p, dir, Some(r_in.time())),
        ))
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Self {
        DiffuseLight { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
    fn emitted(&self, u: f64, v: f64, p: &Position) -> Color {
        self.emit.value(u, v, p)
    }
}
