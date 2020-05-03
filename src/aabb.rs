

use crate::vec3::*;
use crate::ray::*;

#[derive(Debug, Copy, Clone)]
pub struct AABB {
	min: Vec3,
	max: Vec3,
}

impl AABB {
	pub fn new(min: Vec3, max: Vec3) -> AABB {
		AABB{
			min,
			max
		}
	}

	pub fn min(&self) -> Vec3 {
		self.min
	}
	
	pub fn max(&self) -> Vec3 {
		self.max
	}
	
	#[inline]
	pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> bool {
		for i in 0..3 {
			
			let inv_d = 1.0/r.direction().get(i);
			let mut t0 = (self.min.get(i) - r.origin().get(i))*inv_d;
			let mut t1 = (self.max.get(i) - r.origin().get(i))*inv_d;
			if inv_d < 0.0 {
				std::mem::swap(&mut t0, &mut t1);
			}
			let tmin = if t0 > tmin {t0} else {tmin};
			let tmax = if t1 < tmax {t1} else {tmax};
			
			if tmax <= tmin {
				return false;
			}
		}
		true
	}
	
	pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB{
		
		AABB::new(
			Vec3::new(box0.min().x().min(box1.min().x()), //small 
					  box0.min().y().min(box1.min().y()),
					  box0.min().z().min(box1.min().z())), 
					  
			Vec3::new(box0.max().x().max(box1.max().x()), // big
					  box0.max().y().max(box1.max().y()),
					  box0.max().z().max(box1.max().z())))
	}
}