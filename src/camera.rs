use super::Ray;
use super::Vec3;

pub struct Camera {
    pub aspect_ratio: f64,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lower_left: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        const ASPECT_RATIO: f64 = 16. / 9.;
        const VIEWPORT_HEIGHT: f64 = 2.;
        const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f64 = 1.;
        let origin: Vec3 = Vec3::new(0., 0., 0.);
        let horizontal: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0., 0.);
        let vertical: Vec3 = Vec3::new(0., VIEWPORT_HEIGHT, 0.);
        let lower_left: Vec3 =
            origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., FOCAL_LENGTH);
        Camera {
            aspect_ratio: ASPECT_RATIO,
            horizontal,
            vertical,
            origin,
            lower_left,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + u * self.horizontal + v * self.vertical,
        )
    }
}
