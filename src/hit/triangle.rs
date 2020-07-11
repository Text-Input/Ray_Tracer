use crate::aabb::*;
use crate::ray::*;
use crate::vec3::*;
use std::sync::Arc;

use super::*;

// Do we want to have triangles directly, or do we want to store them in some sort of mesh?
// if we could just store references to materials inside objects, I would be happy to have only
// triangles, and no such thing as a mesh. However, using meshes would eliminate most benefits of
// the BVH (when using large meshes), as well as the fact that it would require significant refactoring
// to support hitables only storing a material reference (the main difficulty comes in initializing such a structure,
// due to the fact that once we create an object with a reference to a vector, we cannot add any more elements to it due to
// it requiring a mutable reference).

#[derive(Debug)]
pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,

    edge1: Vec3,
    edge2: Vec3,

    normal: Vec3,

    material: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, material: Arc<dyn Material>) -> Triangle {
        Triangle {
            a,
            b,
            c,
            edge1: b - a,
            edge2: c - a,
            normal: (b - a).cross(c - a),
            material,
        }
    }
}

impl Hitable for Triangle {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        //let edge1 = self.b - self.a;
        //let edge2 = self.c - self.a;

        let edge1 = self.edge1;
        let edge2 = self.edge2;

        let h = r.direction().cross(edge2);
        let a = edge1.dot(h);

        if a > -0.0000001 && a < 0.0000001 {
            // ray is parallel to triangle.
            return None;
        }
        let f = 1.0 / a;
        let s = r.origin() - self.a;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }
        let q = s.cross(edge1);
        let v = f * r.direction().dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        // at this stage ,we comput t (position along vector) to find intersection point.

        let t = f * edge2.dot(q);

        if t > t_min && t < t_max {
            // ray intersection!
            Some(HitRecord {
                t,
                position: r.origin() + r.direction() * t,
                normal: self.normal,
                material: &*self.material,
            })
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        Some(AABB::new(
            Vec3::new(
                a.x().min(b.x()).min(c.x()), //small
                a.y().min(b.y()).min(c.y()),
                a.z().min(b.z()).min(c.z()),
            ),
            Vec3::new(
                a.x().max(b.x()).max(c.x()), // big
                a.y().max(b.y()).max(c.y()),
                a.z().max(b.z()).max(c.z()),
            ),
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
    fn hit_triangle(b: &mut Bencher) {
        let triangle = Triangle::new(
            Vec3::new(0.0, 1.1, 4.2),
            Vec3::new(1.0, 1.1, 5.2),
            Vec3::new(3.0, 1.1, 6.2),
            Box::new(Dielectric::new(1.35)),
        );

        let ray = Ray::new(Vec3::new(0.0, 1.1, 2.2), Vec3::new(2.2, 0.0, 1.1));

        let r = test::black_box(&ray);

        b.iter(|| test::black_box(triangle.hit(r, 0.0, 1000.0)));
    }
}
