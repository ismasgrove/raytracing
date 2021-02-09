use super::{utils, Arc, Perlin, Vec3};
use image::io::Reader as ImageReader;
use image::GenericImageView;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

pub struct Solid {
    color: Vec3,
}

impl Solid {
    fn color_rgb(red: f64, green: f64, blue: f64) -> Self {
        Solid {
            color: Vec3::new(red, green, blue),
        }
    }
    pub fn color_vec3(color: Vec3) -> Self {
        Solid { color }
    }
}

impl Texture for Solid {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        self.color
    }
}

pub struct Checker {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl Checker {
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Checker { odd, even }
    }
    pub fn from_vec3(c1: Vec3, c2: Vec3) -> Self {
        Checker {
            even: Arc::new(Solid::color_vec3(c1)),
            odd: Arc::new(Solid::color_vec3(c2)),
        }
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (10. * p.x()).sin() * (10. * p.y()).sin() * (10. * p.z()).sin();
        if sines < 0. {
            self.odd.value(u, v, &p)
        } else {
            self.even.value(u, v, &p)
        }
    }
}

pub struct Noise {
    noise: Perlin,
    scale: f64,
}

impl Noise {
    pub fn new(scale: f64) -> Self {
        Noise {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for Noise {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        //let p = self.scale * *p;
        //Vec3::new(1., 1., 1.) * 0.5 * (1. + self.noise.generate_noise(&p))
        //Vec3::new(1., 1., 1.) * self.noise.turbulence(&p)
        Vec3::new(1., 1., 1.)
            * 0.5
            * (1. + (self.scale * p.z() + 10. * self.noise.turbulence(p)).sin())
    }
}

pub struct ImageTexture {
    width: u32,
    height: u32,
    bytes_per_scanline: u32,
    bytes_per_pixel: u32,
    image: Vec<u8>,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let bytes_per_pixel = 3;
        let img = ImageReader::open(filename)
            .expect("Image not found.")
            .decode()
            .expect("Decoding failed");

        let (width, height) = (img.width(), img.height());
        let bytes_per_scanline = bytes_per_pixel * width;

        ImageTexture {
            width,
            height,
            bytes_per_scanline,
            bytes_per_pixel,
            image: img.as_bytes().to_vec(),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Vec3) -> Vec3 {
        if self.image.is_empty() {
            Vec3::new(0., 1., 1.)
        } else {
            let (u, v) = (utils::clamp(u, 0., 1.), 1. - utils::clamp(v, 0., 1.));
            let (mut i, mut j) = (
                (u * self.width as f64) as u32,
                (v * self.height as f64) as u32,
            );
            if i >= self.width {
                i = self.width - 1;
            }
            if j >= self.height {
                j = self.height - 1;
            }

            const COLOR_SCALE: f64 = 1. / 255.;
            let pixel_index = (j * self.bytes_per_scanline + i * self.bytes_per_pixel) as usize;

            Vec3::new(
                COLOR_SCALE * self.image[pixel_index] as f64,
                COLOR_SCALE * self.image[pixel_index + 1] as f64,
                COLOR_SCALE * self.image[pixel_index + 2] as f64,
            )
        }
    }
}
