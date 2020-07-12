use crate::aabb::*;
use crate::ray::*;
use crate::vec3::*;
use std::sync::Arc;

use super::*;

#[derive(Debug)]
pub struct XyRectangle {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    material: Arc<dyn Material>,
}

#[derive(Debug)]
pub struct YzRectangle {
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: Arc<dyn Material>,
}

#[derive(Debug)]
pub struct XzRectangle {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: Arc<dyn Material>,
}

impl XyRectangle {
    pub fn new(
        x0: f32,
        x1: f32,
        y0: f32,
        y1: f32,
        k: f32,
        material: Arc<dyn Material>,
    ) -> XyRectangle {
        XyRectangle {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl YzRectangle {
    pub fn new(
        y0: f32,
        y1: f32,
        z0: f32,
        z1: f32,
        k: f32,
        material: Arc<dyn Material>,
    ) -> YzRectangle {
        YzRectangle {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl XzRectangle {
    pub fn new(
        x0: f32,
        x1: f32,
        z0: f32,
        z1: f32,
        k: f32,
        material: Arc<dyn Material>,
    ) -> XzRectangle {
        XzRectangle {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Hitable for XyRectangle {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let outward_normal = Vec3::new(0.0, 0.0, 1.0);

        Some(HitRecord::new(
            t,
            r.point_at_parameter(t),
            r,
            outward_normal,
            &*self.material,
        ))
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

impl Hitable for YzRectangle {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();

        if z < self.z0 || z > self.z1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let outward_normal = Vec3::new(1.0, 0.0, 0.0);

        Some(HitRecord::new(
            t,
            r.point_at_parameter(t),
            r,
            outward_normal,
            &*self.material,
        ))
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}

impl Hitable for XzRectangle {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let outward_normal = Vec3::new(0.0, 1.0, 0.0);

        Some(HitRecord::new(
            t,
            r.point_at_parameter(t),
            r,
            outward_normal,
            &*self.material,
        ))
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}
