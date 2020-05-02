
use crate::colour::*;
use crate::ray::*;
use crate::hit::*;
use crate::material::*;
use crate::util::*;


#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
	albedo: Colour,
}

impl Lambertian {
	pub fn new(albedo: Colour) -> Lambertian {
		Lambertian{
			albedo
		}
	}
}

impl Material for Lambertian {
	fn scatter(&self, _r_in: &Ray, record: &HitRecord) -> Option<MaterialReturn>{
		let target = record.position + record.normal + random_in_unit_sphere();
		let scattered = Ray::new(record.position, target - record.position);
		let attenuation = self.albedo;
		Some(MaterialReturn::new(
			attenuation,
			scattered
		))
	}
}