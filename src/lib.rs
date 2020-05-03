//extern crate hello;
extern crate image;
extern crate rand;

use indicatif::ProgressBar;
use rayon::prelude::*;

pub mod vec3;
pub mod colour;
pub mod ray;
pub mod camera;
mod util;
pub mod material;
pub mod hit;
mod aabb;

use colour::*;
use ray::*;
use camera::*;
use hit::*;
use hit::hitable_list::*;
use hit::bvh::*;

pub fn render(width: usize, height: usize, samples: usize,cam: Camera, world: HitableList) -> Vec<[u8;3]>{
	let bar = ProgressBar::new((height*width) as u64);
	bar.set_style(indicatif::ProgressStyle::default_bar()
					.progress_chars("=> "));
	bar.set_draw_delta((height*width/1000) as u64);
	
	println!("building BVH!");
	//let world = BvhNode::new(world.hitables);
	//println!("BVH tree: {:#?}", world);
	println!("Starting raytracing!");
	
	let buf: Vec<_> = (0..(width*height)).into_par_iter().map(|i| {	
		let x = i%width;
		let y = i/width;
		
		let mut col = Colour::new(0.0, 0.0, 0.0);
		for _ in 0..samples {
			let (r1,r2): (f32, f32) = (rand::random(), rand::random());
			let u = (x as f32 + r1)/(width as f32);
			let v = ((height-y) as f32 + r2)/(height as f32);
		
			let r = cam.get_ray(u,v);
			
			col = col + colour(&r, &world, 0);
		}
		
		col = col/(samples as f32);
		col = Colour::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
		let r = (255.99*col.r()) as u8;
		let g = (255.99*col.g()) as u8;
		let b = (255.99*col.b()) as u8;
		
		bar.inc(1);
		let out: [u8; 3] = [r,g,b];
		out
		//pixel
	}).collect();
	
	bar.finish();
	println!("Done raytracing, finishing up!");
	
	return buf;
	
	
}


pub fn colour(r: &Ray, world: &dyn Hitable, depth: u32) -> Colour {
	match world.hit(r,0.001, f32::MAX) {
		Some(record) => {
			if depth < 50 {
				match record.material.scatter(r, &record) {
					Some(mat) => {
						colour(&mat.scattered(), world, depth+1) * mat.attenuation()
					},
					None => {Colour::new(0.0, 0.0, 0.0)},
				}
			} else {
				Colour::new(0.0, 0.0, 0.0)
			}
		},
		None => {//show background.
			let unit_direction = r.direction().unit_vector();
			let t = 0.6*(unit_direction.y() + 1.0);
			(1.0-t)*Colour::new(1.0, 1.0, 1.0) + t*Colour::new(0.5,0.7,1.0)
		},
	}
}
