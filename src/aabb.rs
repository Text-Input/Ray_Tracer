use crate::ray::*;
use crate::vec3::*;

#[derive(Debug, Copy, Clone)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB { min, max }
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    #[inline]
    pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> bool {
        for i in 0..3 {	
            let inv_d = r.inv_direction().get(i);
			let mut t0 = (self.min.get(i) - r.origin().get(i)) * inv_d;
            let mut t1 = (self.max.get(i) - r.origin().get(i)) * inv_d;
            
			if r.dir_sign()[i] {
                std::mem::swap(&mut t0, &mut t1);
            }
            let tmin = if t0 > tmin { t0 } else { tmin };
            let tmax = if t1 < tmax { t1 } else { tmax };

            if tmax <= tmin {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        AABB::new(
            Vec3::new(
                box0.min().x().min(box1.min().x()), //small
                box0.min().y().min(box1.min().y()),
                box0.min().z().min(box1.min().z()),
            ),
            Vec3::new(
                box0.max().x().max(box1.max().x()), // big
                box0.max().y().max(box1.max().y()),
                box0.max().z().max(box1.max().z()),
            ),
        )
    }

    pub fn longest_axis(&self) -> usize {
        let x_span = self.max().x() - self.min().x();
        let y_span = self.max().y() - self.min().y();
        let z_span = self.max().z() - self.min().z();

        if x_span > y_span && x_span > z_span {
            return 0;
        } else if y_span > z_span {
            return 1;
        } else {
            //z_span is the largest.
            return 2;
        }
    }

    //returns the surface area of the bounding box.
    pub fn area(&self) -> f32 {
        let x_span = self.max().x() - self.min().x();
        let y_span = self.max().y() - self.min().y();
        let z_span = self.max().z() - self.min().z();

        2.0 * (x_span * y_span + x_span * z_span + y_span * z_span)
    }
}
