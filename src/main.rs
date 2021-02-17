extern crate image;
extern crate indicatif;
extern crate rayon;

use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use scenes::two_perlin_spheres;
use std::f64;
use std::sync::Arc;
use std::time::Instant;
use std::{env, sync::Mutex};

mod aabb;
mod bvh;
mod camera;
mod hittable;
mod instance_transforms;
mod material;
mod perlin;
mod pixel;
mod plane;
mod pyramid;
mod ray;
mod rect;
mod scenes;
mod sphere;
mod texture;
mod triangle;
mod utils;
mod vec;
mod volume;

use aabb::AABB;
use bvh::BVHNode;
use camera::Camera;
use hittable::{HitRecord, Hittable, HittableList};
use image::{imageops, ImageFormat, Rgb, RgbImage};
use instance_transforms::{RotateY, Translate};
use material::Material;
use perlin::Perlin;
use plane::Plane;
use pyramid::Pyramid;
use ray::Ray;
use rect::{Cuboid, XYRect, XZRect, YZRect};
use sphere::{MovingSphere, Sphere};
use texture::Texture;
use triangle::Triangle;
use vec::{Color, Direction, Position, Vec3};

fn color<T: Hittable>(r: &Ray, world: &Arc<T>, background: &Color, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
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

fn raytrace<T: Hittable>(
    n_samples: i32,
    nx: i32,
    ny: i32,
    max_depth: i32,
    world: &Arc<T>,
    background: &Color,
    cam: &Camera,
) -> RgbImage {
    let img_arc = Mutex::new(RgbImage::new(nx as u32, ny as u32));
    (0..ny)
        .into_par_iter()
        .progress_count(ny as u64)
        .for_each(|j| {
            for i in 0..nx {
                let mut pixel_color = Color::new(0., 0., 0.);
                for _ in 0..n_samples {
                    let u = (i as f64 + utils::random_double()) / (nx - 1) as f64;
                    let v = (j as f64 + utils::random_double()) / (ny - 1) as f64;
                    pixel_color += color(&cam.get_ray(u, v), world, background, max_depth);
                }
                let mut img_buffer = img_arc.lock().unwrap();
                img_buffer.put_pixel(i as u32, j as u32, Rgb(pixel_color.into_bytes(n_samples)));
            }
        });

    let img = img_arc.into_inner().unwrap();

    imageops::flip_vertical(&img)
}

fn main() {
    let (cam, world, background) = scenes::two_perlin_spheres();
    let nx = 800;
    let ny = (nx as f64 / cam.aspect_ratio) as i32;

    const N_SAMPLES: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    println!("Scanlines processed:");
    assert!(&args[1].contains('.'));

    let filename: Vec<&str> = args[1].split(".").collect();
    let format = match filename[1] {
        "jpeg" | "jpg" => ImageFormat::Jpeg,
        "png" => ImageFormat::Png,
        "bmp" => ImageFormat::Bmp,
        "ico" => ImageFormat::Ico,
        "tiff" => ImageFormat::Tiff,
        _ => panic!("Unsupported format."),
    };

    let start = Instant::now();

    let img_buffer = raytrace(N_SAMPLES, nx, ny, MAX_DEPTH, &world, &background, &cam);

    img_buffer
        .save_with_format(&args[1], format)
        .unwrap_or_default();

    let duration = start.elapsed();

    println!("runtime: {:.2} hours", duration.as_secs_f32() / (60. * 60.));
}
