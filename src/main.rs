use rayon;
use std::f64;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::rc::Rc;

mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec;

use camera::Camera;
use hittable::{HitRecord, Hittable, HittableList};
use material::Material;
use ray::Ray;
use sphere::Sphere;
use vec::Vec3;

fn color(r: &Ray, world: &Rc<dyn Hittable>, depth: i32) -> Vec3 {
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

fn write_color(col: &Vec3, writer: &mut BufWriter<&File>, n_samples: i32) {
    let scale = 1. / n_samples as f64;
    let r = (255.99 * utils::clamp((col.r() * scale).sqrt(), 0., 0.999)) as i32;
    let g = (255.99 * utils::clamp((col.g() * scale).sqrt(), 0., 0.999)) as i32;
    let b = (255.99 * utils::clamp((col.b() * scale).sqrt(), 0., 0.999)) as i32;
    writer
        .write_all(format!("{} {} {}\n", r, g, b).as_bytes())
        .expect("Unable to write");
}

fn main() {
    const NX: f64 = 400.;
    const N_SAMPLES: i32 = 100;
    const MAX_DEPTH: i32 = 50;
    let cam = Camera::new();
    let ny = (NX / cam.aspect_ratio) as i32;

    let f = File::create("picture3.ppm").expect("Unable to create file.");
    let mut writer = BufWriter::new(&f);

    writer
        .write_all(format!("P3\n{} {}\n255\n", NX, ny).as_bytes())
        .expect("Unable to write");

    let material_ground = Rc::new(material::Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(material::Lambertian::new(Vec3::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(material::Metal::new(Vec3::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(material::Metal::new(Vec3::new(0.8, 0.6, 0.2)));

    let world: Rc<dyn Hittable> = Rc::new(HittableList {
        list: vec![
            (Rc::new(Sphere {
                center: Vec3::new(0., -100.5, -1.),
                radius: 100.,
                material: material_ground,
            })),
            Rc::new(Sphere {
                center: Vec3::new(0., 0., -1.),
                radius: 0.5,
                material: material_center,
            }),
            Rc::new(Sphere {
                center: Vec3::new(-1., 0., -1.),
                radius: 0.5,
                material: material_left,
            }),
            Rc::new(Sphere {
                center: Vec3::new(1., 0., -1.),
                radius: 0.5,
                material: material_right,
            }),
        ],
    });

    for j in (0..=(ny - 1) as i32).rev() {
        println!("Scanlines remaining {}", j);
        for i in 0..NX as i32 {
            let mut pixel_color = Vec3::new(1., 0., 0.);
            for _ in 0..N_SAMPLES {
                let u = (i as f64 + utils::random_double()) / (NX - 1.) as f64;
                let v = (j as f64 + utils::random_double()) / (ny - 1) as f64;
                pixel_color += color(&cam.get_ray(u, v), &world, MAX_DEPTH);
            }
            write_color(&pixel_color, &mut writer, N_SAMPLES);
        }
    }
}
