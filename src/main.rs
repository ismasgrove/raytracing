use rand::Rng;
use std::f64;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::rc::Rc;

mod camera;
mod hittable;
mod ray;
mod sphere;
mod utils;
mod vec;

use camera::Camera;
use hittable::{HitRecord, Hittable, HittableList};
use ray::Ray;
use sphere::Sphere;
use vec::Vec3;

fn color(r: &Ray, world: &dyn Hittable<Output = Option<HitRecord>>) -> Vec3 {
    if let Some(hit) = world.hit(r, 0., f64::INFINITY) {
        return 0.5 * (hit.normal + Vec3::new(1., 1., 1.));
    }
    let unit_dir = r.direction().normalize();
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn write_color(col: &Vec3, writer: &mut BufWriter<&File>, n_samples: i32) {
    let scale = 1. / n_samples as f64;
    let r = (255.99 * utils::clamp(col.r() * scale, 0., 0.999)) as i32;
    let g = (255.99 * utils::clamp(col.g() * scale, 0., 0.999)) as i32;
    let b = (255.99 * utils::clamp(col.b() * scale, 0., 0.999)) as i32;
    writer
        .write_all(format!("{} {} {}\n", r, g, b).as_bytes())
        .expect("Unable to write");
}

fn main() {
    const NX: f64 = 800.;
    const N_SAMPLES: i32 = 50;
    let cam = Camera::new();
    let ny = (NX / cam.aspect_ratio) as i32;

    println!("{}", ny);

    let f = File::create("picture.ppm").expect("Unable to create file.");
    let mut writer = BufWriter::new(&f);

    writer
        .write_all(format!("P3\n{} {}\n255\n", NX, ny).as_bytes())
        .expect("Unable to write");

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere {
        center: Vec3::new(0., 0., -1.),
        radius: 0.5,
    }));
    world.add(Rc::new(Sphere {
        center: Vec3::new(0., -100.5, -1.),
        radius: 100.,
    }));

    let mut rd = rand::thread_rng();

    for j in (0..=(ny - 1) as i32).rev() {
        for i in 0..NX as i32 {
            let mut pixel_color = Vec3::new(1., 0., 0.);
            for _ in 0..N_SAMPLES {
                let rd_x: f64 = rd.gen();
                let u = (i as f64 + rd_x) / (NX - 1.) as f64;
                let rd_y: f64 = rd.gen();
                let v = (j as f64 + rd_y) / (ny - 1) as f64;
                pixel_color += color(&cam.get_ray(u, v), &world);
            }
            write_color(&pixel_color, &mut writer, N_SAMPLES);
        }
    }
}
