use rayon::prelude::*;
use std::env;
use std::f64;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Arc;
use std::time::Instant;

mod camera;
mod hittable;
mod material;
mod pixel;
mod ray;
mod sphere;
mod utils;
mod vec;

use camera::Camera;
use hittable::{HitRecord, Hittable, HittableList};
use material::Material;
use pixel::Pixel;
use ray::Ray;
use sphere::Sphere;
use vec::Vec3;

fn color<T: Hittable>(r: &Ray, world: &Arc<T>, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0., 0., 0.);
    }
    if let Some(hit) = world.hit(r, 0.001, f64::INFINITY) {
        let (attenuation, scattered, scatter) = hit.material.scatter(r, &hit);
        if scatter {
            return attenuation * color(&scattered, world, depth - 1);
        } else {
            return Vec3::new(0., 0., 0.);
        }
    }
    let unit_dir = r.direction().normalize();
    let t = 0.5 * (unit_dir.y() + 1.);
    (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn write_pixel(col: &Vec3, writer: &mut BufWriter<&File>, n_samples: i32) {
    let scale = 1. / n_samples as f64;
    let r = (255.99 * utils::clamp((col.r() * scale).sqrt(), 0., 0.999)) as i32;
    let g = (255.99 * utils::clamp((col.g() * scale).sqrt(), 0., 0.999)) as i32;
    let b = (255.99 * utils::clamp((col.b() * scale).sqrt(), 0., 0.999)) as i32;
    writer
        .write_all(format!("{} {} {}\n", r, g, b).as_bytes())
        .expect("Unable to write");
}

fn process_scanline<T: Hittable>(
    scanline_index: i32,
    n_samples: i32,
    nx: i32,
    ny: i32,
    max_depth: i32,
    world: &Arc<T>,
    cam: &Camera,
) -> Vec<Pixel> {
    let mut scanline: Vec<Pixel> = vec![];
    for i in 0..nx as i32 {
        let mut pixel_color = Vec3::new(1., 0., 0.);
        for _ in 0..n_samples {
            let u = (i as f64 + utils::random_double()) / (nx - 1) as f64;
            let v = (scanline_index as f64 + utils::random_double()) / (ny - 1) as f64;
            pixel_color += color(&cam.get_ray(u, v), &world, max_depth);
        }
        scanline.push(Pixel {
            x: i,
            y: scanline_index,
            pixel_color,
        });
    }
    scanline
}

pub fn random_scene() -> HittableList {
    let mut world: HittableList = HittableList { list: vec![] };
    let ground_material = Arc::new(material::Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0., -1000., 0.),
        radius: 1000.,
        material: ground_material,
    }));

    for i in -11..11 {
        for j in -11..11 {
            let choose_mat = utils::random_double();
            let center = Vec3::new(
                i as f64 + 0.9 * utils::random_double(),
                0.2,
                j as f64 + 0.9 * utils::random_double(),
            );

            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_material = Arc::new(material::Lambertian::new(albedo));
                    world.add(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material,
                    }));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_from_range(0.5, 1.);
                    let fuzz = utils::random_from_range(0., 0.5);
                    let sphere_material = Arc::new(material::Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material,
                    }));
                } else {
                    let sphere_material = Arc::new(material::Dielectric::new(1.5));
                    world.add(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material,
                    }));
                }
            }
        }
    }

    world.add(Arc::new(Sphere {
        center: Vec3::new(0., 1., 0.),
        radius: 1.,
        material: Arc::new(material::Dielectric::new(1.5)),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-4., 1., 0.),
        radius: 1.,
        material: Arc::new(material::Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    }));
    world.add(Arc::new(Sphere {
        center: Vec3::new(4., 1., 0.),
        radius: 1.,
        material: Arc::new(material::Metal::new(Vec3::new(0.7, 0.6, 0.5), 1.)),
    }));

    world
}

fn main() {
    const NX: i32 = 1200;
    const N_SAMPLES: i32 = 500;
    const MAX_DEPTH: i32 = 50;
    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let cam = Camera::new(20., lookfrom, lookat, Vec3::new(0., 1., 0.), 0.1, 10.);
    let ny = (NX as f64 / cam.aspect_ratio) as i32;

    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let filename = &args[1];
    assert_eq!(filename.contains(".ppm"), true);
    let f = File::create(filename).expect("Unable to create file.");
    let mut writer = BufWriter::new(&f);

    writer
        .write_all(format!("P3\n{} {}\n255\n", NX, ny).as_bytes())
        .expect("Unable to write");

    let world = Arc::new(random_scene());

    let start = Instant::now();

    let mut image: Vec<Vec<Pixel>> = (0..(ny))
        .into_par_iter()
        .rev()
        .map(|sample| process_scanline(sample, N_SAMPLES, NX, ny, MAX_DEPTH, &world, &cam))
        .collect();

    assert_eq!(image[0].len() * image.len(), (NX as i32 * ny) as usize);
    assert_eq!(
        image[image.len() - 1].len() * image.len(),
        (NX as i32 * ny) as usize
    );

    image.par_sort_unstable();

    for scanline in image.iter() {
        for pixel in scanline.iter() {
            write_pixel(&pixel.color(), &mut writer, N_SAMPLES);
        }
    }

    let duration = start.elapsed();

    println!("runtime: {} minutes", duration.as_secs() / 60);
}
