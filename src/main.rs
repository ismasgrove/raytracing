extern crate image;
extern crate indicatif;
extern crate rayon;

use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::env;
use std::f64;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Arc;
use std::time::Instant;

mod aabb;
mod bvh;
mod camera;
mod hittable;
mod instance_transforms;
mod material;
mod perlin;
mod pixel;
mod ray;
mod rect;
mod scenes;
mod sphere;
mod texture;
mod utils;
mod vec;
mod volume;

use aabb::AABB;
use bvh::BVHNode;
use camera::Camera;
use hittable::{HitRecord, Hittable, HittableList};
use instance_transforms::{RotateY, Translate};
use material::Material;
use perlin::Perlin;
use pixel::Pixel;
use ray::Ray;
use rect::{Cuboid, XYRect, XZRect, YZRect};
use sphere::{MovingSphere, Sphere};
use texture::Texture;
use vec::Vec3;

fn color<T: Hittable>(r: &Ray, world: &Arc<T>, background: &Vec3, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0., 0., 0.);
    }
    if let Some(hit) = world.hit(r, 0.001, f64::INFINITY) {
        let emitted = hit.material.emitted(hit.u, hit.v, &hit.p);
        if let Some((attenuation, scattered)) = hit.material.scatter(r, &hit) {
            emitted + attenuation * color(&scattered, world, background, depth - 1)
        } else {
            emitted
        }
    } else {
        *background
    }
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
    background: &Vec3,
    cam: &Camera,
) -> Vec<Pixel> {
    let mut scanline: Vec<Pixel> = vec![];
    for i in 0..nx as i32 {
        let mut pixel_color = Vec3::new(1., 0., 0.);
        for _ in 0..n_samples {
            let u = (i as f64 + utils::random_double()) / (nx - 1) as f64;
            let v = (scanline_index as f64 + utils::random_double()) / (ny - 1) as f64;
            pixel_color += color(&cam.get_ray(u, v), world, background, max_depth);
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
    const NX: i32 = 800;
    const N_SAMPLES: i32 = 200;
    const MAX_DEPTH: i32 = 50;
    let (cam, world, background) = scenes::final_scene();
    let ny = (NX as f64 / cam.aspect_ratio) as i32;

    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let start = Instant::now();

    println!("Scanlines rendered:");

    let mut image: Vec<Vec<Pixel>> = (0..(ny))
        .into_par_iter()
        .rev()
        .progress_count(ny as u64)
        .map(|sample| {
            process_scanline(
                sample,
                N_SAMPLES,
                NX,
                ny,
                MAX_DEPTH,
                &world,
                &background,
                &cam,
            )
        })
        .collect();

    assert_eq!(image[0].len() * image.len(), (NX as i32 * ny) as usize);
    assert_eq!(
        image[image.len() - 1].len() * image.len(),
        (NX as i32 * ny) as usize
    );

    image.par_sort_unstable();

    let filename = &args[1];
    assert_eq!(filename.contains(".ppm"), true);
    let f = File::create(filename).expect("Unable to create file.");
    let mut writer = BufWriter::new(&f);

    writer
        .write_all(format!("P3\n{} {}\n255\n", NX, ny).as_bytes())
        .expect("Unable to write");

    for scanline in image.iter() {
        for pixel in scanline.iter() {
            write_pixel(&pixel.color(), &mut writer, N_SAMPLES);
        }
    }

    let duration = start.elapsed();

    println!("runtime: {:.2} hours", duration.as_secs_f32() / (60. * 60.));
}
