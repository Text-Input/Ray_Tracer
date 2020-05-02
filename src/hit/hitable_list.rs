

use crate::hit::*;


pub struct HitableList {
	hitables: Vec<Box<dyn Hitable>>,
}

impl HitableList{
	pub fn new(hitables: Vec<Box<dyn Hitable>>) -> HitableList {
		HitableList {
			hitables,
		}
	}
}

impl Hitable for HitableList {
	fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
		let mut closest_so_far = t_max;
		
		let mut rec = None;
		
		for elem in self.hitables.iter() {
			match elem.hit(r, t_min, closest_so_far) {
				Some(record) => {
					closest_so_far = record.t;
					rec = Some(record);
				}
				None => {} //do nothing.
			}
		}
		rec
	}
}