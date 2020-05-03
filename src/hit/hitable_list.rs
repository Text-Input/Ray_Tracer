

use crate::hit::*;

#[derive(Debug)]
pub struct HitableList {
	pub hitables: Vec<Box<dyn Hitable>>,
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
	
	fn bounding_box(&self) -> Option<AABB>{
		if self.hitables.is_empty() {
			return None
		}
		let mut temp_box = self.hitables[0].bounding_box();
		//we account for the first box in temp_box, so we can skip in when iterating though the list.
		for obj in self.hitables.iter().skip(1) { 
			temp_box = match temp_box {
				Some(temp) => {
					match obj.bounding_box() {
						Some(aa_box) => Some(AABB::surrounding_box(temp, aa_box)),
						None => Some(temp),
					}
				},
				None => {
					match obj.bounding_box() {
						Some(aa_box) => Some(aa_box),
						None => None,
					}
				}
				
				
			}
		}
		temp_box
	}
}