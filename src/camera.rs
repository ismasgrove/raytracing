use super::utils;
use super::Ray;
use super::Vec3;

pub struct Camera {
    pub aspect_ratio: f64,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lower_left: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
    time0: Option<f64>,
    time1: Option<f64>,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        vfov: f64,
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        aperture: f64,
        focus_dist: f64,
        time0: Option<f64>,
        time1: Option<f64>,
    ) -> Self {
        let theta = utils::degrees_to_radians(vfov);
        let h = (theta / 2.).tan();
        let viewport_height: f64 = 2. * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left = origin - horizontal / 2. - vertical / 2. - focus_dist * w;
        Camera {
            aspect_ratio,
            horizontal,
            vertical,
            origin,
            lower_left,
            u,
            v,
            lens_radius: aperture / 2.,
            time0,
            time1,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left + s * self.horizontal + t * self.vertical - self.origin - offset,
            Some(utils::random_from_range(
                self.time0.unwrap_or(0.),
                self.time1.unwrap_or(1.),
            )),
        )
    }
}
