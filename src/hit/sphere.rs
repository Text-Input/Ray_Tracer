use crate::hit::*;
use crate::ray::*;
use crate::vec3::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    radius2: f32,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            radius2: radius * radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    //#[inline]
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius2;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;

            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    position: r.point_at_parameter(temp),
                    normal: (r.point_at_parameter(temp) - self.center) / self.radius,
                    material: &*self.material,
                });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    position: r.point_at_parameter(temp),
                    normal: (r.point_at_parameter(temp) - self.center) / self.radius,
                    material: &*self.material,
                });
            }
        }
        None
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}

#[cfg(test)]
mod benches {
    use super::*;

    extern crate test;
    use test::Bencher;

    use crate::material::dielectric::*;

    #[bench]
    fn hit_sphere(b: &mut Bencher) {
        let sphere = Sphere::new(
            Vec3::new(0.0, 1.1, 2.2),
            5.0,
            Box::new(Dielectric::new(1.35)),
        );
        let ray = Ray::new(Vec3::new(0.0, 1.1, 2.2), Vec3::new(2.2, 0.0, 1.1));

        let r = test::black_box(&ray);

        b.iter(|| test::black_box(sphere.hit(r, 0.0, 1000.0)));
    }
}
