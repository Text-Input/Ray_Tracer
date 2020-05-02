
use crate::colour::*;
use crate::ray::*;
use crate::hit::*;
use crate::material::*;
use crate::util::*;


#[derive(Debug, Clone, Copy)]
pub struct Metal {
	albedo: Colour,
	fuzz: f32,
}

impl Metal {
	pub fn new(albedo: Colour, fuzz: f32) -> Metal {
		let mut fuzz = fuzz;
		if fuzz > 1.0 {fuzz = 1.0;}
		if fuzz < 0.0 {fuzz = 0.0;}
		Metal {
			albedo,
			fuzz
		}
	}
}

impl Material for Metal {
	fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<MaterialReturn>{
		let reflected = r_in.direction().unit_vector().reflect(record.normal);
		let scattered = Ray::new(record.position, reflected + self.fuzz*random_in_unit_sphere());
		let attenuation = self.albedo;
		if scattered.direction().dot(record.normal) > 0.0{
			Some(MaterialReturn::new(
				attenuation,
				scattered
			))
		} else {
			None
		}
	}
}