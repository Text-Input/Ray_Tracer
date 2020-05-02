use crate::vec3::*;
use crate::ray::*;
use crate::hit::*;

pub struct Sphere{
	center: Vec3,
	radius: f32,
	material: Box<dyn Material>,
}

impl Sphere {
	pub fn new(center: Vec3, radius: f32, material: Box<dyn Material>) -> Sphere{
		Sphere {
			center,
			radius,
			material,
		}
	}
}

impl Hitable for Sphere {
	fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
		let oc = r.origin() - self.center;
		let a = r.direction().dot(r.direction());
		let b = oc.dot(r.direction());
		let c = oc.dot(oc) - self.radius*self.radius;
		let discriminant = b*b - a*c;
	
		if discriminant > 0.0 {
			let temp = (-b - discriminant.sqrt()) / a;
			
			if temp < t_max && temp > t_min {
				return Some(HitRecord{
					t: temp,
					position: r.point_at_parameter(temp),
					normal: (r.point_at_parameter(temp) - self.center) / self.radius,
					material: &self.material,
				});
			}
			let temp = (-b + discriminant.sqrt()) / a;
			if temp < t_max && temp > t_min {
				return Some(HitRecord{
					t: temp,
					position: r.point_at_parameter(temp),
					normal: (r.point_at_parameter(temp) - self.center) / self.radius,
					material: &self.material,
				});
			}
			
		} 
		None
		
	}
}