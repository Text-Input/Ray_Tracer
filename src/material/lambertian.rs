use crate::colour::*;
use crate::hit::*;
use crate::material::*;
use crate::ray::*;
use crate::util::*;

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, record: &HitRecord) -> Option<MaterialReturn> {
        let scatter_direction = record.normal + random_in_unit_sphere().unit_vector();
        let scattered = Ray::new(record.position, scatter_direction);
        let attenuation = self.albedo;
        Some(MaterialReturn::new(attenuation, scattered))
    }
}
