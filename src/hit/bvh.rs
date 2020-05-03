
use crate::ray::*;
use crate::aabb::*;

use rand::*;

use super::*;

use std::cmp::Ordering;

#[derive(Debug)]
pub struct BvhNode {
	left: Option<Box<dyn Hitable>>,
	right: Option<Box<dyn Hitable>>,
	aa_box: AABB,
}

impl BvhNode {
	
	pub fn new(mut objects: Vec<Box<dyn Hitable>>) -> BvhNode{
		
		let axis = rand::thread_rng().gen_range(0,3);
		
		let comparator = match axis {
			0 => compare_box_by_x_axis,
			1 => compare_box_by_y_axis,
			2 => compare_box_by_z_axis,
			_ => {panic!("Got a random number out of range! Recieved: {}", axis);}
		};
		
		//objects = objects.into_iter()
		//		.sorted_by(|left, right| comparator(left, right))
		//		.collect();
		
		objects.sort_by(|left, right| comparator(left, right));
		
		
		let (left, right) = match objects.len() {
			0 => (None, None),
			1 => (Some(objects.remove(0)), None),
			2 => (Some(objects.remove(0)), Some(objects.remove(0))),
			_ => {
				let mid = objects.len()/2;
				
				let left_objs: Vec<_> = objects.drain(0..mid).collect();
				let right_objs = objects;	
				
				let left: Box<dyn Hitable> = Box::new(BvhNode::new(left_objs));
				let right: Box<dyn Hitable> = Box::new(BvhNode::new(right_objs));
				(Some(left), Some(right))
			}
		};
		
		let bounds = match (&left, &right) {
			(Some(left), Some(right)) => Some(AABB::surrounding_box(left.bounding_box().unwrap(), right.bounding_box().unwrap() )),
			(Some(left), None) => left.bounding_box(),
			(None, Some(right)) => right.bounding_box(),
			(None, None) => None,
		};
		
		let bounds = match bounds {
			Some(bounding_box) => bounding_box,
			None => panic!("Can't have geometry without a bounding box!"),
		};
		
		BvhNode{
			left, right,
			aa_box: bounds
		}
		
	}
}

impl Hitable for BvhNode {
	fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>{
		if !self.aa_box.hit(r, t_min, t_max){
			return None;
		}
		
		let hit_left = match &self.left{
			Some(obj) => obj.hit(r, t_min, t_max),
			None => None
		};
		let hit_right = match &self.right {
			Some(obj) => obj.hit(r, t_min, t_max),
			None => None
		};
		
		match (hit_left, hit_right) {
			(None, None) => None,
			(None, Some(h)) => Some(h),
			(Some(h), None) => Some(h),
			(Some(left), Some(right)) => {
				if left.t < right.t {
					Some(left)
				} else {
					Some(right)
				}
				
			}
		}
		
	}

	fn bounding_box(&self) -> Option<AABB>{
		Some(self.aa_box)
	}
}

fn get_boxes(left: &Box<dyn Hitable>, right: &Box<dyn Hitable> ) -> (AABB, AABB) {
	let l_box = left.bounding_box();
	let r_box = right.bounding_box();
	
	let l_box = match l_box {
		Some(bounds) => bounds,
		None => panic!("Encountered an object with no bounding box!"),
	};

	let r_box = match r_box {
		Some(bounds) => bounds,
		None => panic!("Encountered an object with no bounding box!"),
	};
	(l_box, r_box)
}

fn compare_box_by_x_axis(left: &Box<dyn Hitable>, right: &Box<dyn Hitable>) -> Ordering {
	let (l_box, r_box) = get_boxes(left, right);
	
	l_box.min().x().partial_cmp(&r_box.min().x()).unwrap()
}

fn compare_box_by_y_axis(left: &Box<dyn Hitable>, right: &Box<dyn Hitable>) -> Ordering {
	let (l_box, r_box) = get_boxes(left, right);
	
	l_box.min().z().partial_cmp(&r_box.min().z()).unwrap()	
}

fn compare_box_by_z_axis(left: &Box<dyn Hitable>, right: &Box<dyn Hitable>) -> Ordering {
	let (l_box, r_box) = get_boxes(left, right);
	
	l_box.min().z().partial_cmp(&r_box.min().z()).unwrap()
}