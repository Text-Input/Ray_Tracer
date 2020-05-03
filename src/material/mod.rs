
pub mod dielectric;
pub mod metal;
pub mod lambertian;

use super::colour::*;
use super::ray::*;
use super::hit::*;


pub trait Material: Sync + Send  + std::fmt::Debug{
	fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<MaterialReturn>;
}

#[derive(Debug, Clone, Copy)]
pub struct MaterialReturn{
	attenuation: Colour,
	scattered: Ray,
}

impl MaterialReturn {
	pub fn new(attenuation: Colour, scattered: Ray) -> MaterialReturn{
		MaterialReturn{
			attenuation,
			scattered
		}
	}

	pub fn attenuation(&self) -> Colour {
		self.attenuation
	}
	pub fn scattered(&self) -> Ray{
		self.scattered
	}
}