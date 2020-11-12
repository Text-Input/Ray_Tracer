extern crate ray_tracer;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use std::sync::Arc;

use png::*;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use ray_tracer::camera::*;
use ray_tracer::colour::*;
use ray_tracer::vec3::*;

use ray_tracer::hit::constant_medium::*;
use ray_tracer::hit::hitable_list::*;
use ray_tracer::hit::instancing::*;
use ray_tracer::hit::rectangle::*;
use ray_tracer::hit::sphere::*;
use ray_tracer::hit::triangle::*;
use ray_tracer::hit::*;

use ray_tracer::material::dielectric::*;
use ray_tracer::material::emission::*;
use ray_tracer::material::lambertian::*;
use ray_tracer::material::metal::*;

use std::time::Instant;

fn main() {
    let width = 500*4;
    let height = 500*4;

    //let width = 1920;
    //let height = 1080;

    let samples = 500 ;
    let _seed: u64 = 0;

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

    //cornell box view info
    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        (width as f32) / (height as f32),
        aperture,
        dist_to_focus,
    );

    let mut rng = SmallRng::seed_from_u64(_seed);
    //let scene = random_scene(&mut rng);
    //let scene = static_scene();
    let scene = cornell_box();
    //let scene = simple_light();

    let time_start = Instant::now();

    let buf = ray_tracer::render(width, height, samples, cam, scene);

    let time_end = Instant::now();

    let buf: Vec<[u8; 3]> = buf
        .iter()
        .map(|x| {
            let r = (255.99 * x[0]) as u8;
            let g = (255.99 * x[1]) as u8;
            let b = (255.99 * x[2]) as u8;

            let out: [u8; 3] = [r, g, b];
            out
        })
        .collect();

    let mut buff = Vec::with_capacity(width * height * 3);
    for i in buf.iter() {
        buff.extend(i);
    }
    let file = File::create(Path::new("out.png")).unwrap();
    let mut encoder = Encoder::new(BufWriter::new(file), width as u32, height as u32);
    encoder.set_color(ColorType::RGB);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&buff).unwrap();

    println!(
        "it took {} seconds to render this image",
        time_end.duration_since(time_start).as_secs()
    );
}

#[allow(dead_code)]
fn random_scene(rng: &mut SmallRng) -> HitableList {
    let mut objs: Vec<Box<dyn Hitable>> = vec![];

    objs.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Colour::new(0.5, 0.5, 0.5))),
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
                        Arc::new(Lambertian::new(Colour::new(
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
                        Arc::new(Metal::new(
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
                        Arc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }
    objs.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    objs.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1))),
    )));
    objs.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0)),
    )));

    let tri_mat = Arc::new(Lambertian::new(Colour::new(0.4, 0.4, 0.1)));
    objs.push(Box::new(Triangle::new(
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        tri_mat,
    )));

    HitableList::new(objs)
}

#[allow(dead_code)]
fn static_scene() -> HitableList {
    HitableList::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.5))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Arc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.0))),
        )),
        Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Arc::new(Metal::new(Colour::new(0.8, 0.6, 0.2), 0.3)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Arc::new(Dielectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            Arc::new(Dielectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, 1.3, -1.0),
            0.5,
            Arc::new(Emission::new(Colour::new(1.0, 1.0, 1.0))),
        )),
    ])
}

#[allow(dead_code)]
fn cornell_box() -> HitableList {
    use ray_tracer::material::Material;

    let red: Arc<dyn Material> = Arc::new(Lambertian::new(Colour::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::new(Colour::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::new(Colour::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material> = Arc::new(Emission::new(Colour::new(7.0, 7.0, 7.0)));

    let world: Vec<Box<dyn Hitable>> = vec![
        Box::new(FlipFace::new(YzRectangle::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            Arc::clone(&green),
        ))),
        Box::new(YzRectangle::new(
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
            Arc::clone(&red),
        )),
        Box::new(XzRectangle::new(
            113.0,
            443.0,
            127.0,
            432.0,
            554.0,
            Arc::clone(&light),
        )),
        Box::new(FlipFace::new(XzRectangle::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            Arc::clone(&white),
        ))),
        Box::new(XzRectangle::new(
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
            Arc::clone(&white),
        )),
        Box::new(FlipFace::new(XyRectangle::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            Arc::clone(&white),
        ))),
        /*Box::new(Translate::new(
            RotateY::new(
                RectangularBox::new(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(165.0, 330.0, 165.0),
                    Arc::clone(&white),
                ),
                15.0,
            ),
            Vec3::new(265.0, 0.0, 295.0),
        )),
        Box::new(Translate::new(
            RotateY::new(
                RectangularBox::new(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(165.0, 165.0, 165.0),
                    Arc::clone(&white),
                ),
                -18.0,
            ),
            Vec3::new(130.0, 0.0, 65.0),
        )),*/
        Box::new(ConstantMedium::new(
            Box::new(Translate::new(
                RotateY::new(
                    RectangularBox::new(
                        Vec3::new(0.0, 0.0, 0.0),
                        Vec3::new(165.0, 330.0, 165.0),
                        Arc::clone(&white),
                    ),
                    15.0,
                ),
                Vec3::new(265.0, 0.0, 295.0),
            )),
            0.01,
            Colour::new(0.0, 0.0, 0.0),
        )),
        Box::new(ConstantMedium::new(
            Box::new(Translate::new(
                RotateY::new(
                    RectangularBox::new(
                        Vec3::new(0.0, 0.0, 0.0),
                        Vec3::new(165.0, 165.0, 165.0),
                        Arc::clone(&white),
                    ),
                    -18.0,
                ),
                Vec3::new(130.0, 0.0, 65.0),
            )),
            0.01,
            Colour::new(1.0, 1.0, 1.0),
        )),
    ];

    HitableList::new(world)
}

#[allow(dead_code)]
fn simple_light() -> HitableList {
    HitableList::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::new(Lambertian::new(Colour::new(1.0, 1.0, 1.0))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, 2.0, 0.0),
            2.0,
            Arc::new(Lambertian::new(Colour::new(1.0, 1.0, 1.0))),
        )),
        Box::new(XyRectangle::new(
            3.0,
            5.0,
            1.0,
            3.0,
            -2.0,
            Arc::new(Emission::new(Colour::new(4.0, 4.0, 4.0))),
        )),
    ])
}
