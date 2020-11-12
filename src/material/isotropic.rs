use crate::material::*;
use crate::util::*;
use crate::Colour;

#[derive(Debug)]
pub struct Isotropic {
    albedo: Colour,
}

impl Isotropic {
    pub fn new(albedo: Colour) -> Isotropic {
        Isotropic { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _r_in: &Ray, record: &HitRecord) -> Option<MaterialReturn> {
        Some(MaterialReturn {
            scattered: Ray::new(record.position, random_in_unit_sphere()),
            attenuation: self.albedo,
        })
    }
}
