
use crate::vec3::*;
use crate::colour::*;
use crate::ray::*;
use crate::hit::*;
use crate::material::*;


#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
	ref_idx: f32, //index of refraction.
}

impl Dielectric {
	
	pub fn new(ref_idx: f32) -> Dielectric{
		Dielectric{
			ref_idx
		}
	}

	pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
		let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
		let r0 = r0 * r0;
		r0 + (1.0-r0)*(1.0 - cosine).powi(5)
	}
}

impl Material for Dielectric {
	fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<MaterialReturn>{
		let outward_normal;
		let reflected = r_in.direction().reflect(record.normal);
		let ni_over_nt;
		let attenuation = Colour::new(1.0, 1.0, 1.0);
		let mut refracted = Vec3::new(0.0, 0.0, 0.0); // should get used happen!
		let reflect_prob: f32;
		let cosine: f32;
		
		let scattered;
		
		if r_in.direction().dot(record.normal) > 0.0 {
			outward_normal = -record.normal;
			ni_over_nt = self.ref_idx;
			let cos = r_in.direction().dot(record.normal) / r_in.direction().length();
			cosine = (1.0 - self.ref_idx * self.ref_idx * (1.0 - cos*cos)).sqrt();
		} else {
			outward_normal= record.normal;
			ni_over_nt = 1.0 / self.ref_idx;
			cosine = -r_in.direction().dot(record.normal) / r_in.direction().length();
		}
		match r_in.direction().refract(outward_normal, ni_over_nt) {
			Some(refract) => {
				reflect_prob = Dielectric::schlick(cosine, self.ref_idx);
				refracted = refract;
			},
			None => {
				reflect_prob = 1.0;
			}
		}
		
		if rand::random::<f32>() < reflect_prob {
			scattered = Ray::new(record.position, reflected);
		} else {
			scattered = Ray::new(record.position, refracted);
		}
		Some(MaterialReturn::new(
			attenuation, 
			scattered
		))
	}
}