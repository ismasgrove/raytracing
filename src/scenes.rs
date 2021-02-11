use image::{error::ImageFormatHint, imageops::horizontal_gradient, ImageBuffer};
use material::Lambertian;

use crate::volume::ConstantMedium;

use super::{
    material, texture, utils, Arc, BVHNode, Camera, Cuboid, Hittable, HittableList, MovingSphere,
    RotateY, Sphere, Translate, Vec3, XYRect, XZRect, YZRect,
};

pub fn random_scene() -> (Camera, Arc<HittableList>, Vec3) {
    let mut world: HittableList = HittableList::new(vec![]);
    let checker = Arc::new(texture::Checker::from_vec3(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Arc::new(material::Lambertian::textured(checker));
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
                    let new_center = center + Vec3::new(0., utils::random_from_range(0., 0.5), 0.);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        new_center,
                        0.2,
                        sphere_material,
                        0.,
                        1.,
                    )));
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

    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let cam = Camera::new(
        16. / 9.,
        20.,
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        0.1,
        10.,
        None,
        None,
    );

    (cam, Arc::new(world), Vec3::new(0.7, 0.8, 1.))
}

pub fn two_spheres() -> (Camera, Arc<HittableList>, Vec3) {
    let mut objects = HittableList::new(vec![]);
    let checker = Arc::new(texture::Checker::from_vec3(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    objects.add(Arc::new(Sphere {
        center: Vec3::new(0., -10., 0.),
        radius: 10.,
        material: Arc::new(material::Lambertian::textured(checker)),
    }));

    let checker = Arc::new(texture::Checker::from_vec3(
        Vec3::new(0.6, 0.5, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));

    objects.add(Arc::new(Sphere {
        center: Vec3::new(0., 10., 0.),
        radius: 10.,
        material: Arc::new(material::Lambertian::textured(checker)),
    }));

    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let cam = Camera::new(
        16. / 9.,
        20.,
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        0.0,
        10.,
        None,
        None,
    );

    (cam, Arc::new(objects), Vec3::new(0.7, 0.8, 1.))
}

pub fn two_perlin_spheres() -> (Camera, Arc<HittableList>, Vec3) {
    let mut objects = HittableList::new(vec![]);

    let pertext = texture::Noise::new(4.);

    objects.add(Arc::new(Sphere {
        center: Vec3::new(0., -1000., 0.),
        radius: 1000.,
        material: Arc::new(material::Lambertian::textured(Arc::new(pertext))),
    }));

    let pertext = texture::Noise::new(4.);

    objects.add(Arc::new(Sphere {
        center: Vec3::new(0., 2., 0.),
        radius: 2.,
        material: Arc::new(material::Lambertian::textured(Arc::new(pertext))),
    }));
    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let cam = Camera::new(
        16. / 9.,
        20.,
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        0.0,
        10.,
        None,
        None,
    );

    (cam, Arc::new(objects), Vec3::new(0.7, 0.8, 1.))
}

pub fn globe() -> (Camera, Arc<HittableList>, Vec3) {
    let mut objects = HittableList::new(vec![]);
    let globetext = texture::ImageTexture::new("earthmap.jpg");
    objects.add(Arc::new(Sphere {
        center: Vec3::new(0., 0., 0.),
        radius: 2.,
        material: Arc::new(material::Lambertian::textured(Arc::new(globetext))),
    }));

    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let cam = Camera::new(
        16. / 9.,
        20.,
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        0.0,
        10.,
        None,
        None,
    );

    (cam, Arc::new(objects), Vec3::new(0.7, 0.8, 1.))
}

pub fn simple_light() -> (Camera, Arc<HittableList>, Vec3) {
    let background = Vec3::new(0., 0., 0.);
    let lookfrom = Vec3::new(26., 3., 6.);
    let lookat = Vec3::new(0., 2., 0.);
    let cam = Camera::new(
        16. / 9.,
        20.,
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        0.0,
        10.,
        None,
        None,
    );
    let mut objects = HittableList::new(vec![]);

    let pertext = texture::Noise::new(4.);

    objects.add(Arc::new(Sphere {
        center: Vec3::new(0., -1000., 0.),
        radius: 1000.,
        material: Arc::new(material::Lambertian::textured(Arc::new(pertext))),
    }));

    let pertext = texture::Noise::new(4.);

    objects.add(Arc::new(Sphere {
        center: Vec3::new(0., 2., 0.),
        radius: 2.,
        material: Arc::new(material::Lambertian::textured(Arc::new(pertext))),
    }));

    let difflight = Arc::new(material::DiffuseLight::new(Arc::new(
        texture::Solid::color_vec3(Vec3::new(4., 4., 4.)),
    )));
    objects.add(Arc::new(XYRect::new(3., 5., 1., 3., -2., difflight)));

    (cam, Arc::new(objects), background)
}

pub fn cornell_box() -> (Camera, Arc<HittableList>, Vec3) {
    let background = Vec3::new(0., 0., 0.);
    let lookfrom = Vec3::new(278., 278., -800.);
    let lookat = Vec3::new(278., 278., 0.);
    let cam = Camera::new(
        1.,
        40.,
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        0.0,
        10.,
        None,
        None,
    );

    let mut objects = HittableList::new(vec![]);

    let red = Arc::new(material::Lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(material::Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(material::Lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(material::DiffuseLight::new(Arc::new(
        texture::Solid::color_vec3(Vec3::new(20., 20., 20.)),
    )));

    objects.add(Arc::new(YZRect::new(0., 555., 0., 555., 555., green)));
    objects.add(Arc::new(YZRect::new(0., 555., 0., 555., 0., red)));
    objects.add(Arc::new(XZRect::new(213., 343., 227., 332., 554., light)));
    objects.add(Arc::new(XZRect::new(0., 555., 0., 555., 0., white.clone())));
    objects.add(Arc::new(XZRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    objects.add(Arc::new(XYRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));

    let cuboid1 = Arc::new(Cuboid::new(
        Vec3::new(0., 0., 0.),
        Vec3::new(165., 330., 165.),
        white.clone(),
    ));

    let cuboid1 = Arc::new(RotateY::new(cuboid1, 15.));
    let cuboid1 = Arc::new(Translate::new(cuboid1, Vec3::new(265., 0., 295.)));
    objects.add(cuboid1);

    let cuboid2 = Arc::new(Cuboid::new(
        Vec3::new(0., 0., 0.),
        Vec3::new(165., 165., 165.),
        white.clone(),
    ));

    let cuboid2 = Arc::new(RotateY::new(cuboid2, -18.));
    let cuboid2 = Arc::new(Translate::new(cuboid2, Vec3::new(130., 0., 65.)));
    objects.add(cuboid2);

    (cam, Arc::new(objects), background)
}

pub fn cornell_smoke() -> (Camera, Arc<HittableList>, Vec3) {
    let background = Vec3::new(0., 0., 0.);
    let lookfrom = Vec3::new(278., 278., -800.);
    let lookat = Vec3::new(278., 278., 0.);
    let cam = Camera::new(
        1.,
        40.,
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        0.0,
        10.,
        None,
        None,
    );

    let mut objects = HittableList::new(vec![]);

    let red = Arc::new(material::Lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(material::Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(material::Lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(material::DiffuseLight::new(Arc::new(
        texture::Solid::color_vec3(Vec3::new(7., 7., 7.)),
    )));

    objects.add(Arc::new(YZRect::new(0., 555., 0., 555., 555., green)));
    objects.add(Arc::new(YZRect::new(0., 555., 0., 555., 0., red)));
    objects.add(Arc::new(XZRect::new(113., 443., 127., 432., 554., light)));
    objects.add(Arc::new(XZRect::new(0., 555., 0., 555., 0., white.clone())));
    objects.add(Arc::new(XZRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    objects.add(Arc::new(XYRect::new(
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));

    let cuboid1 = Arc::new(Cuboid::new(
        Vec3::new(0., 0., 0.),
        Vec3::new(165., 330., 165.),
        white.clone(),
    ));

    let cuboid1 = Arc::new(RotateY::new(cuboid1, 15.));
    let cuboid1 = Arc::new(Translate::new(cuboid1, Vec3::new(265., 0., 295.)));
    let cuboid1 = Arc::new(ConstantMedium::new(
        0.01,
        cuboid1,
        Arc::new(texture::Solid::color_vec3(Vec3::new(0., 0., 0.))),
    ));
    objects.add(cuboid1);

    let cuboid2 = Arc::new(Cuboid::new(
        Vec3::new(0., 0., 0.),
        Vec3::new(165., 165., 165.),
        white.clone(),
    ));

    let cuboid2 = Arc::new(RotateY::new(cuboid2, -18.));
    let cuboid2 = Arc::new(Translate::new(cuboid2, Vec3::new(130., 0., 65.)));
    let cuboid2 = Arc::new(ConstantMedium::new(
        0.01,
        cuboid2,
        Arc::new(texture::Solid::color_vec3(Vec3::new(1., 1., 1.))),
    ));
    objects.add(cuboid2);

    (cam, Arc::new(objects), background)
}

pub fn final_scene() -> (Camera, Arc<HittableList>, Vec3) {
    let background = Vec3::new(0., 0., 0.);
    let lookfrom = Vec3::new(478., 278., -600.);
    let lookat = Vec3::new(278., 278., 0.);
    let cam = Camera::new(
        1.,
        40.,
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        0.0,
        10.,
        None,
        None,
    );

    let mut boxes1 = HittableList::new(vec![]);

    let ground = Arc::new(Lambertian::new(Vec3::new(0.48, 0.83, 0.53)));

    const BOXES_PER_SIDE: i32 = 20;

    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.;
            let (x0, y0, z0) = (-1000. + i as f64 * w, 0., -1000. + j as f64 * w);
            let (x1, y1, z1) = (x0 + w, utils::random_from_range(1., 101.), z0 + w);
            boxes1.add(Arc::new(Cuboid::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut objects = HittableList::new(vec![]);

    objects.add(BVHNode::construct_tree(boxes1, 0., 1.));

    let light = Arc::new(material::DiffuseLight::new(Arc::new(
        texture::Solid::color_vec3(Vec3::new(7., 7., 7.)),
    )));
    objects.add(Arc::new(XZRect::new(123., 423., 147., 412., 554., light)));

    let center1 = Vec3::new(400., 400., 200.);
    let center2 = center1 + Vec3::new(30., 0., 0.);
    let moving_sphere_material = Arc::new(material::Lambertian::new(Vec3::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        50.,
        moving_sphere_material,
        0.,
        1.,
    )));

    objects.add(Arc::new(Sphere::new(
        Vec3::new(260., 150., 45.),
        50.,
        Arc::new(material::Dielectric::new(1.5)),
    )));

    objects.add(Arc::new(Sphere::new(
        Vec3::new(0., 150., 145.),
        50.,
        Arc::new(material::Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.)),
    )));

    let boundary = Arc::new(Sphere::new(
        Vec3::new(360., 150., 145.),
        70.,
        Arc::new(material::Dielectric::new(1.5)),
    ));
    objects.add(boundary.clone());

    objects.add(Arc::new(ConstantMedium::new(
        0.2,
        boundary,
        Arc::new(texture::Solid::color_vec3(Vec3::new(0.2, 0.4, 0.9))),
    )));

    let boundary = Arc::new(Sphere::new(
        Vec3::new(0., 0., 0.),
        5000.,
        Arc::new(material::Dielectric::new(1.5)),
    ));

    objects.add(Arc::new(ConstantMedium::new(
        0.0001,
        boundary,
        Arc::new(texture::Solid::color_vec3(Vec3::new(1., 1., 1.))),
    )));

    let globe_material = Arc::new(material::Lambertian::textured(Arc::new(
        texture::ImageTexture::new("earthmap.jpg"),
    )));

    objects.add(Arc::new(Sphere::new(
        Vec3::new(400., 200., 400.),
        100.,
        globe_material,
    )));

    let pertext = Arc::new(texture::Noise::new(0.1));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(220., 280., 300.),
        80.,
        Arc::new(material::Lambertian::textured(pertext)),
    )));

    let mut boxes2 = HittableList::new(vec![]);

    let white = Arc::new(material::Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Vec3::random_from_range(0., 165.),
            10.,
            white.clone(),
        )));
    }

    objects.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(BVHNode::construct_tree(boxes2, 0., 1.), 15.)),
        Vec3::new(-100., 270., 395.),
    )));

    (cam, Arc::new(objects), background)
}
