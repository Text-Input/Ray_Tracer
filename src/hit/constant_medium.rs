use crate::aabb::AABB;
use crate::hit::*;
use crate::material::isotropic::*;
use crate::Colour;

#[derive(Debug)]
pub struct ConstantMedium {
    boundary: Box<dyn Hitable>,
    phase_function: Isotropic,
    neg_inv_density: f32,
}

impl ConstantMedium {
    pub fn new(boundary: Box<dyn Hitable>, density: f32, col: Colour) -> ConstantMedium {
        ConstantMedium {
            boundary,
            phase_function: Isotropic::new(col),
            neg_inv_density: -1.0 / density,
        }
    }
}

/// Note! This assumes that once a ray exits the boundary, it will never reenter.
/// That is, it assumes that the boundary shape is convex.
/// TODO: make this work with concave/any shaped boundaries.
impl Hitable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rec1 = self.boundary.hit(r, f32::MIN, f32::MAX)?;
        let mut rec2 = self.boundary.hit(r, rec1.t + 0.0001, f32::MAX)?;

        if rec1.t < t_min {
            rec1.t = t_min
        }
        if rec2.t > t_max {
            rec2.t = t_max
        }

        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rand::random::<f32>().ln(); //assuming this is supposed to be log_10.

        if hit_distance > distance_inside_boundary {
            return None;
        }

        Some(HitRecord::new(
            rec1.t + hit_distance / ray_length,
            r.point_at_parameter(rec1.t + hit_distance / ray_length),
            r,
            Vec3::new(1.0, 0.0, 0.0), //arbitrary
            &self.phase_function,
        ))
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.boundary.bounding_box()
    }
}
