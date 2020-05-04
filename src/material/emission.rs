
use crate::colour::*;
use super::*;

#[derive(Debug)]
pub struct Emission {
	emit: Colour,
}

impl Emission {
	pub fn new(emit: Colour)-> Emission {
		Emission {
			emit
		}
	}
}

impl Material for Emission {
	fn scatter(&self, _r_in: &Ray, _record: &HitRecord) -> Option<MaterialReturn>{
		None
	}
	
	fn emitted(&self) -> Colour {
		self.emit
	}
}