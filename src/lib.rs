//#![feature(test)]

//extern crate hello;
extern crate rand;

use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use indicatif::ParallelProgressIterator;
use indicatif::ProgressBar;
use rayon::prelude::*;

mod aabb;
pub mod camera;
pub mod colour;
pub mod hit;
pub mod material;
pub mod ray;
mod util;
pub mod vec3;

use camera::*;
use colour::*;
use hit::bvh::*;
use hit::hitable_list::*;
use hit::triangle::*;
use hit::*;
use material::lambertian::*;
use ray::*;
use vec3::*;

pub fn render(
    width: usize,
    height: usize,
    samples: usize,
    cam: Camera,
    world: HitableList,
) -> Vec<[f32; 3]> {
    let bar = ProgressBar::new((height * width) as u64);
    bar.set_style(indicatif::ProgressStyle::default_bar().progress_chars("=> "));
    bar.set_draw_delta((height * width / 1000) as u64);

    println!("building BVH!");
    //let world: BvhNode = BvhNode::new(world.hitables);
    let world: BvhNode = BvhNode::new_sah(world.hitables);
    //let world: Bvh = Bvh::new(world.hitables);

    //world.print_graph();

    println!("Starting raytracing!");

    let buf: Vec<_> = (0..(width * height))
        .into_par_iter()
        .progress_with(bar)
        .map(|i| {
            let x = i % width;
            let y = i / width;
			
            pixel(width, height, x, y, &cam, &world, samples)
            //pixel
        })
        .collect();

    println!("Done raytracing, finishing up!");
	
    buf
}

pub fn pixel(
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    cam: &Camera,
    world: &dyn Hitable,
    samples: usize,
) -> [f32; 3] {
    let mut col = Colour::new(0.0, 0.0, 0.0);
    for _ in 0..samples {
        let (r1, r2): (f32, f32) = (rand::random(), rand::random());
        let u = (x as f32 + r1) / (width as f32);
        let v = ((height - y) as f32 + r2) / (height as f32);

        let r = cam.get_ray(u, v);

        col = col + colour(&r, world, 0);
    }

    col = col / (samples as f32);
    col = Colour::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

    [col.r(), col.g(), col.b()]
}

pub fn colour(r: &Ray, world: &dyn Hitable, depth: u32) -> Colour {
    match world.hit(r, 0.001, f32::MAX) {
        Some(record) => {
            if depth < 50 {
                let emitted = record.material.emitted();

                match record.material.scatter(r, &record) {
                    Some(mat) => {
                        emitted + colour(&mat.scattered(), world, depth + 1) * mat.attenuation()
                    }
                    None => emitted,
                }
            } else {
                Colour::new(0.0, 0.0, 0.0)
            }
        }
        None => {
            //show background.
            //let unit_direction = r.direction().unit_vector();
            //let t = 0.6 * (unit_direction.y() + 1.0);
            //(1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)

            Colour::new(0.0, 0.0, 0.0) // make the background be solid black.
        }
    }
}

#[pyfunction]
fn py_render(
    width: usize,
    height: usize,
    samples: usize,
    cam: Camera,
    world: Vec<Vec<Vec3>>,
) -> Vec<[f32; 4]> {
    let mat: Arc<dyn material::Material> = Arc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    let l: Vec<_> = world
        .iter()
        .map(|x| Box::new(Triangle::new(x[0], x[1], x[2], Arc::clone(&mat))) as Box<dyn Hitable>)
        .collect();

    println!("length of triangles: {}", l.len());
    let rend = render(width, height, samples, cam, HitableList::new(l));

    println!("{}, {}, {}", rend[0][0], rend[0][1], rend[0][2]);

    let mut out: Vec<[f32; 3]> = Vec::with_capacity(width * height);

    //blender needs an image flipped on the horizontal axis.
    for i in 0..height {
        out.extend_from_slice(
            &rend[(width * height - i * width - width)..(width * height - i * width)],
        )
    }

    out.iter().map(|x| [(x[0]), (x[1]), (x[2]), 1.0]).collect()
}

#[pymodule]
fn ray_tracer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(py_render))?;
    m.add_class::<Camera>()?;
    m.add_class::<Vec3>()?;

    Ok(())
}

#[cfg(test)]
mod benches {
    use super::*;

    use std::sync::Arc;

    extern crate test;
    use test::Bencher;

    use crate::hit::sphere::*;
    use crate::material::dielectric::*;

    #[bench]
    fn color_1sphere(b: &mut Bencher) {
        let sphere = Sphere::new(
            Vec3::new(0.0, 3.0, 0.0),
            1.0,
            Arc::new(Dielectric::new(1.35)),
        );
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

        let r = test::black_box(&ray);

        b.iter(|| test::black_box(colour(r, &sphere, 48)));
    }

    #[bench]
    fn color_bvh_1sphere(b: &mut Bencher) {
        let bvh = BvhNode::new(vec![Box::new(Sphere::new(
            Vec3::new(0.0, 3.0, 0.0),
            1.0,
            Arc::new(Dielectric::new(1.35)),
        ))]);

        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

        let r = test::black_box(&ray);

        b.iter(|| test::black_box(colour(r, &bvh, 0)));
    }
}
