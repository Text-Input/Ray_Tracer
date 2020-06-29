use super::ray::*;
use super::vec3::*;

use super::util::*;

use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}
#[pymethods]
impl Camera {
    //vfov is top to bottom in degrees.
    #[new]
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        Camera {
            //lower_left_corner: Vec3::new(-half_width, -half_height, -1.0),
            lower_left_corner: (lookfrom
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w),
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: lookfrom,
            u: u,
            v: v,
            w: w,
            lens_radius: (aperture / 2.0),
        }
    }
}

impl Camera {
    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + x * self.horizontal + y * self.vertical - self.origin - offset,
        )
    }
}
