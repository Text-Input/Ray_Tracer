pub mod dielectric;
pub mod emission;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

use super::colour::*;
use super::hit::*;
use super::ray::*;

pub trait Material: Sync + Send + std::fmt::Debug {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<MaterialReturn>;

    //by default, emit no light.
    fn emitted(&self) -> Colour {
        Colour::new(0.0, 0.0, 0.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MaterialReturn {
    attenuation: Colour,
    scattered: Ray,
}

impl MaterialReturn {
    pub fn new(attenuation: Colour, scattered: Ray) -> MaterialReturn {
        MaterialReturn {
            attenuation,
            scattered,
        }
    }

    pub fn attenuation(&self) -> Colour {
        self.attenuation
    }
    pub fn scattered(&self) -> Ray {
        self.scattered
    }
}
