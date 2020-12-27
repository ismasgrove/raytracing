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

fn main() {
    const NX: i32 = 1000;
    const N_SAMPLES: i32 = 100;
    const MAX_DEPTH: i32 = 100;
    let cam = Camera::new();
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

    let material_ground = Arc::new(material::Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(material::Lambertian::new(Vec3::new(0.7, 0.3, 0.3)));
    let material_left = Arc::new(material::Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(material::Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    let world = Arc::new(HittableList::new(vec![
        Arc::new(Sphere {
            center: Vec3::new(0., -100.5, -1.),
            radius: 100.,
            material: material_ground,
        }),
        Arc::new(Sphere {
            center: Vec3::new(0., 0., -1.),
            radius: 0.5,
            material: material_center,
        }),
        Arc::new(Sphere {
            center: Vec3::new(-1., 0., -1.),
            radius: 0.5,
            material: material_left,
        }),
        Arc::new(Sphere {
            center: Vec3::new(1., 0., -1.),
            radius: 0.5,
            material: material_right,
        }),
    ]));
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

    println!("runtime: {}", duration.as_secs());
}
