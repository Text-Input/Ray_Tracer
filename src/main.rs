extern crate ray_tracer;

use image::ImageBuffer;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use ray_tracer::camera::*;
use ray_tracer::colour::*;
use ray_tracer::vec3::*;

use ray_tracer::hit::hitable_list::*;
use ray_tracer::hit::sphere::*;
use ray_tracer::hit::triangle::*;
use ray_tracer::hit::*;

use ray_tracer::material::dielectric::*;
use ray_tracer::material::emission::*;
use ray_tracer::material::lambertian::*;
use ray_tracer::material::metal::*;

use std::time::Instant;

fn main() {
    let width = 400 * 2;
    let height = 200 * 2;
    let samples = 200 * 8;
    let seed: u64 = 0;

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (width as f32) / (height as f32),
        aperture,
        dist_to_focus,
    );

    //let mut rng = SmallRng::seed_from_u64(seed);
    //let scene = random_scene(&mut rng);
    let scene = static_scene();

    let time_start = Instant::now();

    let buf = ray_tracer::render(width, height, samples, cam, scene);

    let time_end = Instant::now();

    let mut buff = Vec::with_capacity(width * height * 3);
    for i in buf.iter() {
        buff.extend(i);
    }

    let imgbuf: ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_vec(width as u32, height as u32, buff).unwrap();

    imgbuf.save("out.png").unwrap();

    println!(
        "it took {} seconds to render this image",
        time_end.duration_since(time_start).as_secs()
    );
}

fn random_scene(rng: &mut SmallRng) -> HitableList {
    let mut objs: Vec<Box<dyn Hitable>> = vec![];

    objs.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(Colour::new(0.5, 0.5, 0.5))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3::new(
                (a as f32) + 0.9 * rng.gen::<f32>(),
                0.2,
                (b as f32) + 0.9 * rng.gen::<f32>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse

                    objs.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Lambertian::new(Colour::new(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        ))),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    objs.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Metal::new(
                            Colour::new(
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                            ),
                            0.0,
                        )),
                    )));
                } else {
                    //glass
                    objs.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }
    objs.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    )));
    objs.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1))),
    )));
    objs.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0)),
    )));

    //let tri_mat = Box::new(Lambertian::new(Colour::new(0.4, 0.4, 0.1)));
    //objs.push(Box::new(Triangle::new(Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), tri_mat)));

    HitableList::new(objs)
}

fn static_scene() -> HitableList {
    let world = HitableList::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Box::new(Lambertian::new(Colour::new(0.1, 0.2, 0.5))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(Lambertian::new(Colour::new(0.8, 0.8, 0.0))),
        )),
        Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Box::new(Metal::new(Colour::new(0.8, 0.6, 0.2), 0.3)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Box::new(Dielectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            Box::new(Dielectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, 1.3, -1.0),
            0.5,
            Box::new(Emission::new(Colour::new(1.0, 1.0, 1.0))),
        )),
    ]);
    world
}
